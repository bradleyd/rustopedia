use anyhow::{Context, Result};
use serde_json::Value;
use std::time::Instant;

use crate::config::AppConfig;
use crate::generate_prompt::{build_prompt, format_agent_output_for_llm, review_intent};
use crate::llm::LlmClient;
use crate::planner::{PlanStep, generate_plan};
use crate::session::{Command, ParsedInput, SessionMode, SessionState, parse_input};

pub struct Runtime {
    config: AppConfig,
    llm_client: LlmClient,
}

struct TurnArtifacts {
    plan: Vec<PlanStep>,
    context_chunks: Vec<String>,
    review_intent: Option<String>,
}

impl Runtime {
    pub fn new(config: AppConfig) -> Result<Self> {
        let llm_client = LlmClient::new(config.clone()).context("failed to initialize runtime")?;
        Ok(Self { config, llm_client })
    }

    pub fn banner(&self) -> String {
        "Rustopedia (commands: /mode ask|review|edit, /status, /help, exit)".to_string()
    }

    pub async fn handle_input(
        &self,
        input: &str,
        session: &mut SessionState,
    ) -> Result<HandleResult> {
        match parse_input(input) {
            ParsedInput::Exit => Ok(HandleResult::Exit),
            ParsedInput::Command(command) => {
                Ok(HandleResult::Message(self.handle_command(command, session)))
            }
            ParsedInput::Query(query) if query.is_empty() => Ok(HandleResult::Noop),
            ParsedInput::Query(query) => {
                let mode = session.mode();
                tokio::select! {
                    result = self.execute_turn(query, mode, session) => {
                        let response = result?;
                        session.push_turn(query.to_string(), response.trim().to_string());
                        Ok(HandleResult::Message(response))
                    }
                    signal = tokio::signal::ctrl_c() => {
                        match signal {
                            Ok(()) => Ok(HandleResult::Message("Interrupted current request.".to_string())),
                            Err(e) => Err(e).context("failed to listen for Ctrl-C"),
                        }
                    }
                }
            }
        }
    }

    fn handle_command(&self, command: Command<'_>, session: &mut SessionState) -> String {
        match command {
            Command::Help => [
                "/mode ask       answer questions with local + external context",
                "/mode review    explain or review code/questions without editing files",
                "/mode edit      plan edits and code-writing flow scaffolding",
                "/status         show current mode and history count",
                "exit            quit the session",
            ]
            .join("\n"),
            Command::Status => format!(
                "mode: {}\nhistory_turns: {}",
                session.mode().as_str(),
                session.history().len()
            ),
            Command::Mode(mode) => {
                session.set_mode(mode);
                format!("mode set to {}", mode.as_str())
            }
            Command::Unknown(raw) => format!("unknown command: {raw}"),
        }
    }

    async fn execute_turn(
        &self,
        query: &str,
        mode: SessionMode,
        session: &SessionState,
    ) -> Result<String> {
        let turn_started = Instant::now();
        let artifacts = self.gather_context(query, mode).await;
        let synth_started = Instant::now();
        let prompt = self.synthesize_prompt(query, mode, session, &artifacts);
        let synth_elapsed = synth_started.elapsed();
        self.log_prompt_debug(query, mode, &artifacts, &prompt);
        self.log_stage_timing("synthesize", synth_elapsed);
        let generation_started = Instant::now();
        let response = self.generate_response(&prompt).await;
        self.log_stage_timing("generate", generation_started.elapsed());
        self.log_stage_timing("turn_total", turn_started.elapsed());
        response
    }

    async fn gather_context(&self, query: &str, mode: SessionMode) -> TurnArtifacts {
        let review_intent = match mode {
            SessionMode::Review => Some(review_intent(query).to_string()),
            _ => None,
        };

        let analyze_started = Instant::now();
        let mut context_chunks = self
            .gather_local_workspace_context(mode, review_intent.as_deref())
            .await;
        self.log_stage_timing("analyze", analyze_started.elapsed());

        let route_started = Instant::now();
        let plan = self.route_tools(query, mode).await;
        self.log_stage_timing("route", route_started.elapsed());

        let execute_started = Instant::now();
        context_chunks.extend(self.execute_tools(&plan).await);
        self.log_stage_timing("execute", execute_started.elapsed());

        let retrieve_started = Instant::now();
        context_chunks.extend(self.retrieve_memory(query, &plan).await);
        self.log_stage_timing("retrieve", retrieve_started.elapsed());

        TurnArtifacts {
            plan,
            context_chunks,
            review_intent,
        }
    }

    async fn gather_local_workspace_context(
        &self,
        mode: SessionMode,
        review_intent: Option<&str>,
    ) -> Vec<String> {
        match mode {
            SessionMode::Ask => Vec::new(),
            SessionMode::Review | SessionMode::Edit => {
                println!("Stage: analyze ({})", mode.as_str());
                let mut context_chunks = Vec::new();

                let project_overview_started = Instant::now();
                match crate::tools::project_overview::summarize_project(&self.config).await {
                    Ok(summary) => context_chunks.push(summary),
                    Err(e) => eprintln!("⚠️ project overview failed: {e}"),
                }
                self.log_stage_timing("project_overview", project_overview_started.elapsed());

                if should_run_rust_analyzer(mode, review_intent) {
                    let rust_analyzer_started = Instant::now();
                    match crate::tools::rust_analyzer::workspace_summary(&self.config).await {
                        Ok(summary) => context_chunks.push(summary),
                        Err(e) => eprintln!("⚠️ rust-analyzer analysis failed: {e}"),
                    }
                    self.log_stage_timing("rust_analyzer", rust_analyzer_started.elapsed());
                } else if self.config.debug {
                    eprintln!(
                        "[debug] stage=rust_analyzer skipped=true intent={}",
                        review_intent.unwrap_or("n/a")
                    );
                }

                context_chunks
            }
        }
    }

    async fn route_tools(&self, query: &str, mode: SessionMode) -> Vec<PlanStep> {
        if matches!(mode, SessionMode::Review) {
            println!("Stage: route ({})", mode.as_str());
            println!(
                "Review mode uses local project analysis first; skipping planner/tool routing."
            );
            return Vec::new();
        }

        println!("Stage: route ({})", mode.as_str());
        match generate_plan(query, &self.config.planner_model_name, &self.llm_client).await {
            Ok(plan) => {
                println!("Plan {:?}", plan);
                plan
            }
            Err(e) => {
                eprintln!("⚠️ Planner failed: {e}");
                Vec::new()
            }
        }
    }

    async fn execute_tools(&self, plan: &[PlanStep]) -> Vec<String> {
        println!("Stage: execute");
        let mut context_chunks = Vec::new();

        for step in plan {
            println!("Running agent: {} with input: {}", step.tool, step.input);
            match run_agent(&step.tool, &step.input, &self.config).await {
                Ok(Some(response)) => match serde_json::from_str::<Value>(&response) {
                    Ok(json_value) => {
                        context_chunks.push(format_agent_output_for_llm(&step.tool, &json_value))
                    }
                    Err(_) => {
                        context_chunks.push(format!("From {}: {}", step.tool, response.trim()))
                    }
                },
                Ok(None) => eprintln!("⚠️ Unknown agent in plan: {}", step.tool),
                Err(e) => eprintln!("⚠️ Agent failed ({}): {e}", step.tool),
            }
        }

        context_chunks
    }

    async fn retrieve_memory(&self, query: &str, plan: &[PlanStep]) -> Vec<String> {
        if plan.is_empty() {
            println!("Stage: retrieve");
            println!("No RAG collections selected.");
            return Vec::new();
        }

        println!("Stage: retrieve");
        let mut context_chunks = Vec::new();
        let mut rag_collections = std::collections::HashSet::new();

        for step in plan {
            match step.tool.as_str() {
                "crate_agent" => {
                    rag_collections.insert("crates");
                }
                "docs_agent" => {
                    rag_collections.insert("rust-docs");
                }
                "github_agent" => {
                    rag_collections.insert("rust-book");
                }
                _ => {}
            }
        }
        for collection in rag_collections {
            match call_rag(query, collection, self.config.rag_top_k).await {
                Ok(rag) if !rag.trim().is_empty() => {
                    context_chunks.push(format!("From memory ({}): {}", collection, rag.trim()));
                }
                Ok(_) => eprintln!("⚠️ Empty RAG response for collection '{}'.", collection),
                Err(e) => eprintln!("⚠️ {e}"),
            }
        }

        context_chunks
    }

    fn synthesize_prompt(
        &self,
        query: &str,
        mode: SessionMode,
        session: &SessionState,
        artifacts: &TurnArtifacts,
    ) -> String {
        println!("Stage: synthesize");
        let past = session
            .history()
            .iter()
            .map(|turn| format!("User: {}\nAssistant: {}", turn.query, turn.response))
            .collect::<Vec<String>>()
            .join("\n");

        let full_context = artifacts.context_chunks.join("\n\n");
        build_prompt(mode, &past, &full_context, query, &artifacts.plan)
    }

    fn log_prompt_debug(
        &self,
        query: &str,
        mode: SessionMode,
        artifacts: &TurnArtifacts,
        prompt: &str,
    ) {
        if !self.config.debug {
            return;
        }

        let context_chars = artifacts
            .context_chunks
            .iter()
            .map(|chunk| chunk.chars().count())
            .sum::<usize>();
        let query_tokens = estimate_tokens(query);
        let context_tokens = estimate_tokens_from_chars(context_chars);
        let prompt_tokens = estimate_tokens(prompt);

        eprintln!(
            "[debug] mode={} intent={} query_chars={} query_tokens~={} plan_steps={} context_chunks={} context_chars={} context_tokens~={} prompt_chars={} prompt_tokens~={}",
            mode.as_str(),
            artifacts.review_intent.as_deref().unwrap_or("n/a"),
            query.chars().count(),
            query_tokens,
            artifacts.plan.len(),
            artifacts.context_chunks.len(),
            context_chars,
            context_tokens,
            prompt.chars().count(),
            prompt_tokens
        );
    }

    fn log_stage_timing(&self, stage: &str, elapsed: std::time::Duration) {
        if !self.config.debug {
            return;
        }

        eprintln!("[debug] stage={} elapsed_ms={}", stage, elapsed.as_millis());
    }

    async fn generate_response(&self, prompt: &str) -> Result<String> {
        self.llm_client
            .generate(&self.config.model_name, prompt)
            .await
            .context("failed to generate final response")
    }
}

fn estimate_tokens(text: &str) -> usize {
    estimate_tokens_from_chars(text.chars().count())
}

fn estimate_tokens_from_chars(chars: usize) -> usize {
    ((chars as f64) / 4.0).ceil() as usize
}

fn should_run_rust_analyzer(mode: SessionMode, review_intent: Option<&str>) -> bool {
    match mode {
        SessionMode::Ask => false,
        SessionMode::Edit => true,
        SessionMode::Review => matches!(
            review_intent,
            Some("evaluative_review" | "code_path_explanation")
        ),
    }
}

async fn run_agent(agent_type: &str, query: &str, config: &AppConfig) -> Result<Option<String>> {
    match agent_type {
        "crate_agent" => crate::tools::crate_search::search_crates(query, config)
            .await
            .map(|json| Some(serde_json::to_string_pretty(&json).unwrap_or_default()))
            .map_err(|e| anyhow::anyhow!("crate_agent request failed: {e}")),
        "docs_agent" => crate::tools::docs::search_docs(query, config)
            .await
            .map(|json| Some(serde_json::to_string_pretty(&json).unwrap_or_default()))
            .map_err(|e| anyhow::anyhow!("docs_agent request failed: {e}")),
        "github_agent" => crate::tools::github::search_github(query, config)
            .await
            .map(|json| Some(serde_json::to_string_pretty(&json).unwrap_or_default()))
            .map_err(|e| anyhow::anyhow!("github_agent request failed: {e}")),
        _ => Ok(None),
    }
}

async fn call_rag(query: &str, collection: &str, top_k: usize) -> Result<String> {
    crate::tools::qdrant_client::query_qdrant_with_text(collection, query, top_k)
        .await
        .map_err(|e| anyhow::anyhow!("Qdrant query failed for collection '{collection}': {e}"))
}

pub enum HandleResult {
    Exit,
    Message(String),
    Noop,
}
