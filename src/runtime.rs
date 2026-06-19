use anyhow::{Context, Result};
use serde_json::Value;
use std::time::Instant;

use crate::config::AppConfig;
use crate::generate_prompt::format_agent_output_for_llm;
use crate::intents::{
    RustIntent, clarification_prompt, edit_request_needs_clarification, infer_initial_intent,
};
use crate::llm::LlmClient;
use crate::memory::{CurrentTask, MemoryState, WorkingMemoryItem};
use crate::planner::{PlanStep, generate_plan};
use crate::session::{
    Command, MemorySnapshot, ParsedInput, PendingClarification, SessionMode, SessionState,
    SubjectAnchor, parse_input,
};

pub struct Runtime {
    config: AppConfig,
    llm_client: LlmClient,
}

struct TurnArtifacts {
    plan: Vec<PlanStep>,
    working_memory: Vec<WorkingMemoryItem>,
    initial_intent: RustIntent,
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
            ParsedInput::Query("") => Ok(HandleResult::Noop),
            ParsedInput::Query(query) => {
                let mode = session.mode();
                let initial_intent = infer_initial_intent(query, mode, session.history());
                if let Some(question) = clarification_prompt(query, mode, initial_intent) {
                    if self.config.debug {
                        eprintln!(
                            "[debug] mode={} intent={} clarification=true",
                            mode.as_str(),
                            initial_intent.as_str()
                        );
                    }
                    session.set_pending_clarification(initial_intent, query.to_string());
                    session.push_turn(query.to_string(), question.to_string());
                    return Ok(HandleResult::Message(question.to_string()));
                }

                let pending = session.take_pending_clarification();
                let followup = contextual_followup(query, session.last_subject());
                let effective_query =
                    merge_query_context(query, pending.as_ref(), followup.as_ref());
                let forced_intent = pending
                    .as_ref()
                    .map(|pending| pending.intent)
                    .or_else(|| followup.as_ref().map(|followup| followup.intent));
                tokio::select! {
                    result = self.execute_turn(
                        &effective_query,
                        mode,
                        session,
                        forced_intent.or(Some(initial_intent)),
                        followup.is_some(),
                    ) => {
                        let outcome = result?;
                        let response = outcome.response;
                        session.push_turn(query.to_string(), response.trim().to_string());
                        session.set_last_subject(
                            mode,
                            forced_intent.unwrap_or(initial_intent),
                            effective_query.clone(),
                        );
                        session.set_last_memory_snapshot(outcome.memory_snapshot);
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

    /// Apply already-anchor-verified patches into a scratch git worktree and
    /// run `cargo check` there. Returns `Ok(None)` when the post-patch tree
    /// compiles cleanly, `Ok(Some(evidence))` when it does not, and `Err`
    /// when the validation infrastructure itself failed (e.g. project is
    /// not a git repo, worktree creation failed) — in which case the
    /// caller should fall through without retrying.
    async fn validate_via_scratch(
        &self,
        verified: &crate::patch::VerifiedPatches,
    ) -> Result<Option<crate::retry_loop::RetryEvidence>> {
        let project_root = std::path::PathBuf::from(&self.config.project_root);
        let shared_target = project_root.join("target");

        let validate_started = Instant::now();
        let overlay = tokio::task::spawn_blocking({
            let project_root = project_root.clone();
            move || crate::scratch::ScratchOverlay::create(&project_root)
        })
        .await
        .context("scratch overlay task panicked")??;

        let overlay_root = overlay.root().to_path_buf();

        let verified_for_apply = verified.clone();
        let overlay = tokio::task::spawn_blocking(move || {
            overlay.apply(&verified_for_apply).map(|()| overlay)
        })
        .await
        .context("scratch overlay apply task panicked")??;

        let outcome = crate::tools::cargo::cargo_check_at(
            &overlay_root,
            Some(&shared_target),
            &self.config,
        )
        .await?;

        self.log_stage_timing("scratch_validate", validate_started.elapsed());

        if outcome.ok {
            drop(overlay);
            return Ok(None);
        }

        let prior_patch = crate::patch::render_canonical_patches(verified);
        let evidence = crate::retry_loop::gather_validation_evidence(
            &outcome.output_excerpt,
            &overlay_root,
            &prior_patch,
        )
        .await;
        drop(overlay);

        Ok(Some(evidence))
    }

    /// Non-interactive single-prompt entry point used by the CLI flag path.
    ///
    /// Skips the REPL's pending-clarification machinery (one-shot has no
    /// follow-up turn) but otherwise drives the same `execute_turn` pipeline
    /// the REPL uses. The returned summary lets the CLI emit structured
    /// output and set exit codes.
    pub async fn handle_one_shot(
        &self,
        query: &str,
        mode: SessionMode,
    ) -> Result<OneShotSummary> {
        let started = Instant::now();
        let mut session = SessionState::new();
        session.set_mode(mode);
        let initial_intent = infer_initial_intent(query, mode, session.history());

        let outcome = self
            .execute_turn(query, mode, &session, Some(initial_intent), false)
            .await?;

        let parsed = crate::patch::parse(&outcome.response);
        let project_root = std::path::Path::new(&self.config.project_root);
        let verified = crate::patch::verify(&parsed, project_root);
        let (ok, blocked) = count_anchor_status(&verified);

        Ok(OneShotSummary {
            mode,
            message: outcome.response,
            iterations: outcome.iterations,
            max_retries: if mode == SessionMode::Edit {
                self.config.edit_max_retries
            } else {
                0
            },
            patches_proposed: verified.patches.len(),
            patches_applicable: ok,
            anchor_failures: blocked,
            elapsed_ms: started.elapsed().as_millis(),
        })
    }

    fn handle_command(&self, command: Command<'_>, session: &mut SessionState) -> String {
        match command {
            Command::Help => [
                "/mode ask       Rust-first Q&A with minimal docs/crates/examples fallback",
                "/mode review    inspect or critique the local Rust workspace",
                "/mode edit      plan or drive Rust code changes and verification",
                "/status         show current mode and last-turn memory summary",
                "/memory         show the last-turn memory layers in detail",
                "exit            quit the session",
            ]
            .join("\n"),
            Command::Status => {
                let mode = session.mode();
                let profile = self.config.profile_for_mode(mode);
                let mut lines = vec![
                    format!("mode: {}", mode.as_str()),
                    format!("history_turns: {}", session.history().len()),
                    format!(
                        "model: {} ({}, context~{}t)",
                        profile.id,
                        profile.provider.as_str(),
                        profile.context_window_tokens
                    ),
                ];

                if let Some(snapshot) = session.last_memory_snapshot() {
                    lines.push(format!("last_intent: {}", snapshot.intent.as_str()));
                    lines.push(format!(
                        "memory: {} items ({} file, {} diff, {} text)",
                        snapshot.working_memory_items,
                        snapshot.file_excerpts,
                        snapshot.diff_items,
                        snapshot.text_items
                    ));
                }

                lines.join("\n")
            }
            Command::Memory => {
                if let Some(snapshot) = session.last_memory_snapshot() {
                    let mut lines = vec![
                        format!("mode: {}", snapshot.mode.as_str()),
                        format!("intent: {}", snapshot.intent.as_str()),
                        format!("working_memory_items: {}", snapshot.working_memory_items),
                        format!("file_excerpts: {}", snapshot.file_excerpts),
                        format!("diff_items: {}", snapshot.diff_items),
                        format!("text_items: {}", snapshot.text_items),
                        format!("session_summary_chars: {}", snapshot.session_summary_chars),
                        format!(
                            "background_summary_chars: {}",
                            snapshot.background_summary_chars
                        ),
                    ];

                    if snapshot.item_summaries.is_empty() {
                        lines.push("items: none".to_string());
                    } else {
                        lines.push("items:".to_string());
                        lines.extend(
                            snapshot
                                .item_summaries
                                .iter()
                                .map(|item| format!("- {item}")),
                        );
                    }

                    lines.join("\n")
                } else {
                    "no memory snapshot yet; run a query first".to_string()
                }
            }
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
        forced_intent: Option<RustIntent>,
        contextual_followup: bool,
    ) -> Result<TurnOutcome> {
        let initial_intent =
            forced_intent.unwrap_or_else(|| infer_initial_intent(query, mode, session.history()));

        let turn_started = Instant::now();
        let artifacts = self
            .gather_context(query, mode, initial_intent, contextual_followup)
            .await;
        if let Some(question) = post_analysis_clarification(query, mode, initial_intent, &artifacts)
        {
            if self.config.debug {
                eprintln!(
                    "[debug] mode={} intent={} clarification=true post_analysis=true",
                    mode.as_str(),
                    initial_intent.as_str()
                );
            }
            let memory_snapshot = snapshot_memory_state(query, mode, session, &artifacts);
            return Ok(TurnOutcome {
                response: question.to_string(),
                memory_snapshot,
                iterations: 0,
            });
        }
        let memory_snapshot = snapshot_memory_state(query, mode, session, &artifacts);
        let model_name = self.config.model_for_mode(mode);
        let max_retries: u32 = if mode == SessionMode::Edit {
            self.config.edit_max_retries
        } else {
            0
        };

        let mut iteration: u32 = 0;
        let mut retry_directive: Option<String> = None;
        let mut total_generation_elapsed = std::time::Duration::ZERO;
        let mut last_prompt_tokens: usize;
        let last_response: String;
        let mut prev_was_format_failure = false;
        // Keep the best attempt across retries: a later iteration can degrade (prose
        // drift, malformed patch) and must not overwrite a better earlier result.
        let mut best_response: Option<String> = None;
        let mut best_quality: i64 = i64::MIN;
        let mut best_iteration: u32 = 0;

        loop {
            let synth_started = Instant::now();
            let prompt =
                self.synthesize_prompt(query, mode, session, &artifacts, retry_directive.as_deref());
            let synth_elapsed = synth_started.elapsed();
            last_prompt_tokens = estimate_tokens(&prompt);
            self.log_prompt_debug(query, mode, model_name, &artifacts, &prompt);
            self.log_stage_timing("synthesize", synth_elapsed);

            let generation_started = Instant::now();
            let response = self.generate_response(model_name, &prompt).await?;
            let generation_elapsed = generation_started.elapsed();
            total_generation_elapsed += generation_elapsed;
            if iteration == 0 {
                self.log_stage_timing("generate", generation_elapsed);
            } else {
                self.log_stage_timing(
                    &format!("generate_retry_{iteration}"),
                    generation_elapsed,
                );
            }

            if mode != SessionMode::Edit {
                last_response = response;
                break;
            }

            let parsed = crate::patch::parse(&response);
            let project_root = std::path::Path::new(&self.config.project_root);
            let verified = crate::patch::verify(&parsed, project_root);

            let (ok_now, blocked_now) = count_anchor_status(&verified);
            if self.config.debug {
                let placeholders_now = crate::patch::count_placeholder_hits(&verified);
                eprintln!(
                    "[debug] retry_loop iter={} verified: patches={} errors={} edits_ok={} edits_blocked={} placeholders={}",
                    iteration,
                    verified.patches.len(),
                    verified.errors.len(),
                    ok_now,
                    blocked_now,
                    placeholders_now
                );
            }

            let quality = attempt_quality(
                verified.patches.len(),
                ok_now,
                blocked_now,
                verified.errors.len(),
            );
            if quality > best_quality {
                best_quality = quality;
                best_response = Some(response.clone());
                best_iteration = iteration;
            }

            // Format errors trump every other retry kind. If the model
            // produced no parseable patches at all (either a malformed
            // block or no patch fence whatsoever), reset format
            // expectations explicitly. A response with at least one
            // usable patch — even alongside malformed siblings — falls
            // through to anchor/validation so partial progress is kept.
            // Circuit-break on two consecutive format failures — the
            // model is stuck on the envelope and another retry won't
            // help.
            let is_format_failure = parsed.patches.is_empty();
            if is_format_failure {
                if prev_was_format_failure {
                    if self.config.debug {
                        eprintln!(
                            "[debug] retry_loop circuit_break iteration={} reason=consecutive_format_failure",
                            iteration
                        );
                    }
                    last_response = response;
                    break;
                }
                if iteration < max_retries {
                    let evidence =
                        crate::retry_loop::gather_format_evidence(&parsed, &response);
                    if self.config.debug {
                        eprintln!(
                            "[debug] retry_loop scheduling iteration={} kind=format parse_errors={} patches={} max_retries={}",
                            iteration + 1,
                            parsed.errors.len(),
                            parsed.patches.len(),
                            max_retries
                        );
                    }
                    retry_directive =
                        Some(crate::retry_loop::build_retry_directive(query, &evidence));
                    iteration += 1;
                    prev_was_format_failure = true;
                    continue;
                }
                // Budget exhausted on a format failure — fall through.
                last_response = response;
                break;
            }

            prev_was_format_failure = false;

            let anchor_retryable = crate::retry_loop::has_retryable_failures(&verified);
            if iteration < max_retries && anchor_retryable {
                let evidence = match crate::retry_loop::gather_retry_evidence(
                    &verified,
                    &self.config,
                )
                .await
                {
                    Ok(ev) => ev,
                    Err(err) => {
                        eprintln!("⚠️ retry evidence gathering failed: {err}");
                        last_response = response;
                        break;
                    }
                };

                if !evidence.is_empty() {
                    if self.config.debug {
                        eprintln!(
                            "[debug] retry_loop scheduling iteration={} kind=anchor retryable_hits={} max_retries={}",
                            iteration + 1,
                            evidence.hits.len(),
                            max_retries
                        );
                    }
                    retry_directive =
                        Some(crate::retry_loop::build_retry_directive(query, &evidence));
                    iteration += 1;
                    continue;
                }
            }

            // Anchors clean (or anchor retries exhausted). If every emitted
            // patch is applicable, validate the result by applying it in a
            // scratch worktree and running `cargo check` there.
            let (ok_count, blocked_count) = count_anchor_status(&verified);
            let all_applicable =
                blocked_count == 0 && ok_count > 0 && !verified.patches.is_empty();
            if iteration < max_retries && all_applicable {
                match self.validate_via_scratch(&verified).await {
                    Ok(None) => {
                        // Compiles cleanly in the scratch worktree — definitively the
                        // best possible attempt, regardless of prior scores.
                        best_iteration = iteration;
                        best_response = Some(response.clone());
                        last_response = response;
                        break;
                    }
                    Ok(Some(evidence)) => {
                        if self.config.debug {
                            eprintln!(
                                "[debug] retry_loop scheduling iteration={} kind=cargo_check retryable_hits={} max_retries={}",
                                iteration + 1,
                                evidence.hits.len(),
                                max_retries
                            );
                        }
                        retry_directive = Some(crate::retry_loop::build_retry_directive(
                            query, &evidence,
                        ));
                        iteration += 1;
                        continue;
                    }
                    Err(err) => {
                        if self.config.debug {
                            eprintln!("[debug] scratch validation skipped: {err:#}");
                        }
                        last_response = response;
                        break;
                    }
                }
            }

            last_response = response;
            break;
        }

        let chosen = best_response.unwrap_or(last_response);
        if self.config.debug && best_iteration != iteration {
            eprintln!(
                "[debug] retry_loop emitting best attempt from iteration={} (final iteration={})",
                best_iteration, iteration
            );
        }
        let response = self.append_edit_outputs(
            mode,
            chosen,
            model_name,
            total_generation_elapsed,
            turn_started.elapsed(),
            last_prompt_tokens,
            iteration,
            max_retries,
        );
        self.log_stage_timing("turn_total", turn_started.elapsed());
        Ok(TurnOutcome {
            response,
            memory_snapshot,
            iterations: iteration,
        })
    }

    fn append_edit_outputs(
        &self,
        mode: SessionMode,
        response: String,
        model_name: &str,
        generation_elapsed: std::time::Duration,
        turn_elapsed: std::time::Duration,
        prompt_tokens: usize,
        retries_used: u32,
        max_retries: u32,
    ) -> String {
        if mode != SessionMode::Edit {
            return response;
        }

        let parsed = crate::patch::parse(&response);
        let project_root = std::path::Path::new(&self.config.project_root);
        let verified = crate::patch::verify(&parsed, project_root);
        let preview = crate::patch::render_preview(&verified);

        let (ok, blocked) = count_anchor_status(&verified);
        let placeholder_hits = crate::patch::count_placeholder_hits(&verified);

        if self.config.debug {
            eprintln!(
                "[debug] stage=patch_preview patches={} errors={} edits_ok={} edits_blocked={} placeholders={} retries={}/{}",
                verified.patches.len(),
                verified.errors.len(),
                ok,
                blocked,
                placeholder_hits,
                retries_used,
                max_retries
            );
        }

        let summary = build_edit_run_summary(EditRunMetrics {
            model_name,
            provider: self.config.llm_provider.as_str(),
            generation_ms: generation_elapsed.as_millis(),
            turn_total_ms: turn_elapsed.as_millis(),
            prompt_tokens,
            response_chars: response.chars().count(),
            response_tokens: estimate_tokens(&response),
            patches_parsed: verified.patches.len(),
            parse_errors: verified.errors.len(),
            edits_ok: ok,
            edits_blocked: blocked,
            placeholder_hits,
            retries_used,
            max_retries,
        });

        let mut sections = vec![response.trim_end().to_string()];
        if let Some(preview) = preview {
            sections.push(preview);
        }
        sections.push(summary);
        sections.join("\n\n")
    }

    async fn gather_context(
        &self,
        query: &str,
        mode: SessionMode,
        initial_intent: RustIntent,
        contextual_followup: bool,
    ) -> TurnArtifacts {
        let analyze_started = Instant::now();
        let mut working_memory = self
            .gather_local_workspace_context(query, mode, initial_intent)
            .await;
        self.log_stage_timing("analyze", analyze_started.elapsed());

        let route_started = Instant::now();
        let plan = self
            .route_tools(
                query,
                mode,
                initial_intent,
                contextual_followup,
                &working_memory,
            )
            .await;
        self.log_stage_timing("route", route_started.elapsed());

        let execute_started = Instant::now();
        working_memory.extend(self.execute_tools(&plan).await);
        self.log_stage_timing("execute", execute_started.elapsed());

        let retrieve_started = Instant::now();
        working_memory.extend(
            self.retrieve_memory(query, mode, initial_intent, &plan)
                .await,
        );
        self.log_stage_timing("retrieve", retrieve_started.elapsed());

        TurnArtifacts {
            plan,
            working_memory,
            initial_intent,
        }
    }

    async fn gather_local_workspace_context(
        &self,
        query: &str,
        mode: SessionMode,
        intent: RustIntent,
    ) -> Vec<WorkingMemoryItem> {
        match mode {
            SessionMode::Ask => {
                if !should_run_project_overview(mode, query, intent)
                    && !should_run_targeted_local_context(mode, intent)
                {
                    return Vec::new();
                }

                println!("Stage: analyze ({})", mode.as_str());
                let mut working_memory = Vec::new();

                if should_run_targeted_local_context(mode, intent) {
                    match crate::tools::local_context::gather_relevant_code_excerpts(
                        query,
                        &self.config,
                    )
                    .await
                    {
                        Ok(excerpts) => {
                            working_memory
                                .extend(excerpts.into_iter().map(WorkingMemoryItem::FileExcerpt));
                        }
                        Err(e) => eprintln!("⚠️ local context failed: {e}"),
                    }
                }

                if should_run_project_overview(mode, query, intent) {
                    match crate::tools::project_overview::summarize_project(&self.config).await {
                        Ok(summary) => working_memory.push(WorkingMemoryItem::Text {
                            label: "Project Overview".to_string(),
                            content: summary,
                        }),
                        Err(e) => eprintln!("⚠️ project overview failed: {e}"),
                    }
                }

                working_memory
            }
            SessionMode::Review | SessionMode::Edit => {
                println!("Stage: analyze ({})", mode.as_str());
                let mut working_memory = Vec::new();

                if mode == SessionMode::Edit {
                    match crate::tools::edit_context::gather_edit_context(query, &self.config).await
                    {
                        Ok(edit_context) => {
                            working_memory.push(WorkingMemoryItem::Text {
                                label: "Likely Edit Targets".to_string(),
                                content: edit_context.target_summary,
                            });
                            working_memory.extend(
                                edit_context
                                    .excerpts
                                    .into_iter()
                                    .map(WorkingMemoryItem::FileExcerpt),
                            );
                            if let Some(facts) = current_code_facts(&working_memory) {
                                working_memory.push(WorkingMemoryItem::Text {
                                    label: "Current Code Facts".to_string(),
                                    content: facts,
                                });
                            }
                            if let Some(skeletons) = file_skeletons(
                                &working_memory,
                                std::path::Path::new(&self.config.project_root),
                            ) {
                                working_memory.push(WorkingMemoryItem::Text {
                                    label: "File Skeletons".to_string(),
                                    content: skeletons,
                                });
                            }
                        }
                        Err(e) => eprintln!("⚠️ edit context failed: {e}"),
                    }

                    match crate::tools::diff::workspace_diff_summary(&self.config).await {
                        Ok(Some(diff)) => working_memory.push(WorkingMemoryItem::DiffSummary(diff)),
                        Ok(None) => {}
                        Err(e) => eprintln!("⚠️ workspace diff failed: {e}"),
                    }
                }

                if should_run_rust_analyzer(mode, intent) {
                    let rust_analyzer_started = Instant::now();
                    match crate::tools::rust_analyzer::workspace_summary(&self.config).await {
                        Ok(summary) => working_memory.push(WorkingMemoryItem::Text {
                            label: "Rust Analyzer".to_string(),
                            content: summary,
                        }),
                        Err(e) => eprintln!("⚠️ rust-analyzer analysis failed: {e}"),
                    }
                    self.log_stage_timing("rust_analyzer", rust_analyzer_started.elapsed());
                } else if self.config.debug {
                    eprintln!(
                        "[debug] stage=rust_analyzer skipped=true intent={}",
                        intent.as_str()
                    );
                }

                if should_run_cargo_check(mode, intent) {
                    let cargo_check_started = Instant::now();
                    match crate::tools::cargo::cargo_check_summary(&self.config).await {
                        Ok(summary) => working_memory.push(WorkingMemoryItem::Text {
                            label: "Cargo Check".to_string(),
                            content: summary,
                        }),
                        Err(e) => eprintln!("⚠️ cargo check failed: {e}"),
                    }
                    self.log_stage_timing("cargo_check", cargo_check_started.elapsed());
                } else if self.config.debug {
                    eprintln!(
                        "[debug] stage=cargo_check skipped=true intent={}",
                        intent.as_str()
                    );
                }

                if should_run_cargo_clippy(mode, intent) {
                    let cargo_clippy_started = Instant::now();
                    match crate::tools::cargo::cargo_clippy_summary(&self.config).await {
                        Ok(summary) => working_memory.push(WorkingMemoryItem::Text {
                            label: "Cargo Clippy".to_string(),
                            content: summary,
                        }),
                        Err(e) => eprintln!("⚠️ cargo clippy failed: {e}"),
                    }
                    self.log_stage_timing("cargo_clippy", cargo_clippy_started.elapsed());
                } else if self.config.debug {
                    eprintln!(
                        "[debug] stage=cargo_clippy skipped=true intent={}",
                        intent.as_str()
                    );
                }

                if should_run_cargo_test(mode, intent) {
                    let cargo_test_started = Instant::now();
                    match crate::tools::cargo::cargo_test_summary(&self.config).await {
                        Ok(summary) => working_memory.push(WorkingMemoryItem::Text {
                            label: "Cargo Test".to_string(),
                            content: summary,
                        }),
                        Err(e) => eprintln!("⚠️ cargo test failed: {e}"),
                    }
                    self.log_stage_timing("cargo_test", cargo_test_started.elapsed());
                } else if self.config.debug {
                    eprintln!(
                        "[debug] stage=cargo_test skipped=true intent={}",
                        intent.as_str()
                    );
                }

                let project_overview_started = Instant::now();
                match crate::tools::project_overview::summarize_project(&self.config).await {
                    Ok(summary) => working_memory.push(WorkingMemoryItem::Text {
                        label: "Project Overview".to_string(),
                        content: summary,
                    }),
                    Err(e) => eprintln!("⚠️ project overview failed: {e}"),
                }
                self.log_stage_timing("project_overview", project_overview_started.elapsed());

                if mode == SessionMode::Review && intent == RustIntent::EvaluativeReview {
                    prioritize_evaluative_review_memory(&mut working_memory);
                }
                if mode == SessionMode::Edit {
                    prioritize_edit_memory(&mut working_memory);
                }

                working_memory
            }
        }
    }

    async fn route_tools(
        &self,
        query: &str,
        mode: SessionMode,
        intent: RustIntent,
        contextual_followup: bool,
        working_memory: &[WorkingMemoryItem],
    ) -> Vec<PlanStep> {
        println!("Stage: route ({})", mode.as_str());
        if contextual_followup {
            println!("Contextual follow-up detected; skipping external tool routing.");
            return Vec::new();
        }
        // Edit mode derives docs.rs lookups from the just-gathered workspace context;
        // other modes use the static intent-based plan.
        let plan = match mode {
            SessionMode::Edit => self.derive_edit_doc_plan(query, working_memory),
            _ => generate_plan(query, mode, intent),
        };
        if plan.is_empty() {
            println!("Rust-first routing selected no external tools.");
        } else {
            println!("Rust-first plan {:?} for intent {}", plan, intent.as_str());
        }
        plan
    }

    /// Derive docs.rs lookups for an edit turn from the workspace context already
    /// gathered in the analyze stage. Pure string processing — no LLM, no extra network
    /// calls (the only fetch is the resulting `docs_agent` step in `execute_tools`).
    fn derive_edit_doc_plan(
        &self,
        query: &str,
        working_memory: &[WorkingMemoryItem],
    ) -> Vec<PlanStep> {
        let project_root = std::path::Path::new(&self.config.project_root);

        // Distinct `.rs` files referenced by the edit excerpts (same collection shape as
        // `file_skeletons`), re-read in full so the `use` parser sees imports.
        let mut seen_paths: Vec<String> = Vec::new();
        for item in working_memory {
            if let WorkingMemoryItem::FileExcerpt(excerpt) = item
                && excerpt.path.ends_with(".rs")
                && !seen_paths.iter().any(|p| p == &excerpt.path)
            {
                seen_paths.push(excerpt.path.clone());
            }
        }
        let sources: Vec<String> = seen_paths
            .iter()
            .filter_map(|relative| std::fs::read_to_string(project_root.join(relative)).ok())
            .collect();
        if sources.is_empty() {
            return Vec::new();
        }

        let deps = match std::fs::read_to_string(project_root.join("Cargo.toml")) {
            Ok(toml) => crate::tools::project_overview::parse_dependency_names(&toml),
            Err(_) => Vec::new(),
        };

        let ranked = crate::tools::doc_targets::derive_targets(query, &sources, &deps);
        let selected: Vec<String> = ranked
            .iter()
            .take(crate::tools::doc_targets::MAX_TARGETS)
            .cloned()
            .collect();

        if selected.is_empty() {
            return Vec::new();
        }

        let dropped = &ranked[selected.len()..];
        println!(
            "Edit doc-targets selected {:?}{}",
            selected,
            if dropped.is_empty() {
                String::new()
            } else {
                format!(" (dropped {dropped:?})")
            }
        );

        selected
            .into_iter()
            .map(|input| PlanStep {
                tool: "docs_agent".to_string(),
                input,
            })
            .collect()
    }

    async fn execute_tools(&self, plan: &[PlanStep]) -> Vec<WorkingMemoryItem> {
        println!("Stage: execute");
        let mut working_memory = Vec::new();

        for step in plan {
            println!("Running agent: {} with input: {}", step.tool, step.input);
            match run_agent(&step.tool, &step.input, &self.config).await {
                Ok(Some(response)) => match serde_json::from_str::<Value>(&response) {
                    Ok(json_value) => working_memory.push(WorkingMemoryItem::Text {
                        label: format!("Tool Output ({})", step.tool),
                        content: format_agent_output_for_llm(&step.tool, &json_value),
                    }),
                    Err(_) => working_memory.push(WorkingMemoryItem::Text {
                        label: format!("Tool Output ({})", step.tool),
                        content: format!("From {}: {}", step.tool, response.trim()),
                    }),
                },
                Ok(None) => eprintln!("⚠️ Unknown agent in plan: {}", step.tool),
                Err(e) => eprintln!("⚠️ Agent failed ({}): {e}", step.tool),
            }
        }

        working_memory
    }

    async fn retrieve_memory(
        &self,
        query: &str,
        mode: SessionMode,
        intent: RustIntent,
        plan: &[PlanStep],
    ) -> Vec<WorkingMemoryItem> {
        if !should_run_retrieve(mode, intent, plan) {
            println!("Stage: retrieve");
            println!(
                "Skipping legacy semantic retrieval for mode={} intent={}.",
                mode.as_str(),
                intent.as_str()
            );
            return Vec::new();
        }

        if plan.is_empty() {
            println!("Stage: retrieve");
            println!("No RAG collections selected.");
            return Vec::new();
        }

        println!("Stage: retrieve");
        let mut working_memory = Vec::new();
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
                    working_memory.push(WorkingMemoryItem::Text {
                        label: format!("Retrieved Memory ({collection})"),
                        content: format!("From memory ({}): {}", collection, rag.trim()),
                    });
                }
                Ok(_) => eprintln!("⚠️ Empty RAG response for collection '{}'.", collection),
                Err(e) => eprintln!("⚠️ {e}"),
            }
        }

        working_memory
    }

    fn synthesize_prompt(
        &self,
        query: &str,
        mode: SessionMode,
        session: &SessionState,
        artifacts: &TurnArtifacts,
        retry_directive: Option<&str>,
    ) -> String {
        println!("Stage: synthesize");
        let memory = MemoryState {
            current_task: CurrentTask {
                mode,
                intent: artifacts.initial_intent,
                query: query.to_string(),
            },
            working_memory: artifacts.working_memory.clone(),
            session_summary: trimmed_history(session.history(), 4, 2_000),
            background_summary: None,
        };

        crate::generate_prompt::build_prompt_with_retry(&memory, &artifacts.plan, retry_directive)
    }

    fn log_prompt_debug(
        &self,
        query: &str,
        mode: SessionMode,
        model_name: &str,
        artifacts: &TurnArtifacts,
        prompt: &str,
    ) {
        if !self.config.debug {
            return;
        }

        let context_chars = artifacts
            .working_memory
            .iter()
            .map(WorkingMemoryItem::char_count)
            .sum::<usize>();
        let query_tokens = estimate_tokens(query);
        let context_tokens = estimate_tokens_from_chars(context_chars);
        let prompt_tokens = estimate_tokens(prompt);

        eprintln!(
            "[debug] mode={} intent={} model={} query_chars={} query_tokens~={} plan_steps={} context_chunks={} context_chars={} context_tokens~={} prompt_chars={} prompt_tokens~={}",
            mode.as_str(),
            artifacts.initial_intent.as_str(),
            model_name,
            query.chars().count(),
            query_tokens,
            artifacts.plan.len(),
            artifacts.working_memory.len(),
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

    async fn generate_response(&self, model_name: &str, prompt: &str) -> Result<String> {
        self.llm_client
            .generate(model_name, prompt)
            .await
            .context("failed to generate final response")
    }
}

struct TurnOutcome {
    response: String,
    memory_snapshot: MemorySnapshot,
    iterations: u32,
}

fn snapshot_memory_state(
    query: &str,
    mode: SessionMode,
    session: &SessionState,
    artifacts: &TurnArtifacts,
) -> MemorySnapshot {
    let current_task = CurrentTask {
        mode,
        intent: artifacts.initial_intent,
        query: query.to_string(),
    };
    let memory = MemoryState {
        current_task,
        working_memory: artifacts.working_memory.clone(),
        session_summary: trimmed_history(session.history(), 4, 2_000),
        background_summary: None,
    };

    let file_excerpts = memory
        .working_memory
        .iter()
        .filter(|item| matches!(item, WorkingMemoryItem::FileExcerpt(_)))
        .count();
    let diff_items = memory
        .working_memory
        .iter()
        .filter(|item| matches!(item, WorkingMemoryItem::DiffSummary(_)))
        .count();
    let text_items = memory
        .working_memory
        .len()
        .saturating_sub(file_excerpts + diff_items);

    MemorySnapshot {
        mode,
        intent: artifacts.initial_intent,
        working_memory_items: memory.working_memory.len(),
        file_excerpts,
        diff_items,
        text_items,
        session_summary_chars: memory.session_summary.chars().count(),
        background_summary_chars: memory
            .background_summary
            .as_deref()
            .unwrap_or("")
            .chars()
            .count(),
        item_summaries: memory
            .working_memory
            .iter()
            .map(WorkingMemoryItem::summary_label)
            .collect(),
    }
}

fn estimate_tokens(text: &str) -> usize {
    estimate_tokens_from_chars(text.chars().count())
}

fn estimate_tokens_from_chars(chars: usize) -> usize {
    ((chars as f64) / 4.0).ceil() as usize
}

fn should_run_project_overview(mode: SessionMode, query: &str, intent: RustIntent) -> bool {
    match mode {
        SessionMode::Ask => {
            matches!(
                intent,
                RustIntent::LocalWorkspaceQuestion
                    | RustIntent::ProjectExplanation
                    | RustIntent::ArchitectureSummary
                    | RustIntent::CodePathExplanation
            ) || query.to_ascii_lowercase().contains("workspace")
        }
        SessionMode::Review | SessionMode::Edit => true,
    }
}

fn should_run_targeted_local_context(mode: SessionMode, intent: RustIntent) -> bool {
    match mode {
        SessionMode::Ask => matches!(
            intent,
            RustIntent::LocalWorkspaceQuestion | RustIntent::CodePathExplanation
        ),
        SessionMode::Review | SessionMode::Edit => false,
    }
}

fn should_run_rust_analyzer(mode: SessionMode, intent: RustIntent) -> bool {
    match mode {
        SessionMode::Ask => false,
        SessionMode::Edit => matches!(intent, RustIntent::Refactor),
        SessionMode::Review => matches!(intent, RustIntent::CodePathExplanation),
    }
}

fn should_run_cargo_check(mode: SessionMode, intent: RustIntent) -> bool {
    match mode {
        SessionMode::Ask => false,
        SessionMode::Edit => true,
        SessionMode::Review => matches!(intent, RustIntent::EvaluativeReview),
    }
}

fn should_run_cargo_clippy(mode: SessionMode, intent: RustIntent) -> bool {
    match mode {
        SessionMode::Ask => false,
        SessionMode::Edit => true,
        SessionMode::Review => matches!(intent, RustIntent::EvaluativeReview),
    }
}

fn should_run_cargo_test(mode: SessionMode, intent: RustIntent) -> bool {
    match mode {
        SessionMode::Ask => false,
        SessionMode::Edit => true,
        SessionMode::Review => matches!(intent, RustIntent::EvaluativeReview),
    }
}

fn should_run_retrieve(mode: SessionMode, intent: RustIntent, plan: &[PlanStep]) -> bool {
    if plan.is_empty() {
        return false;
    }

    match mode {
        SessionMode::Ask => false,
        SessionMode::Review | SessionMode::Edit => matches!(
            intent,
            RustIntent::ApiLookup | RustIntent::ExampleLookup | RustIntent::CrateSelection
        ),
    }
}

fn post_analysis_clarification(
    query: &str,
    mode: SessionMode,
    intent: RustIntent,
    artifacts: &TurnArtifacts,
) -> Option<&'static str> {
    if mode != SessionMode::Edit
        || intent != RustIntent::FeatureImplementation
        || !edit_request_needs_clarification(query)
    {
        return None;
    }

    if has_concrete_edit_signal(&artifacts.working_memory) {
        return None;
    }

    Some(
        "What is actually wrong right now: a compiler error, failing test, clippy warning, or a behavior bug?",
    )
}

fn has_concrete_edit_signal(working_memory: &[WorkingMemoryItem]) -> bool {
    working_memory.iter().any(|item| match item {
        WorkingMemoryItem::Text { label, content } if label == "Cargo Check" => {
            !content.contains("succeeded with no compiler errors")
        }
        WorkingMemoryItem::Text { label, content } if label == "Cargo Clippy" => {
            !content.contains("succeeded with no lint warnings")
        }
        WorkingMemoryItem::Text { label, content } if label == "Cargo Test" => {
            !content.contains("all tests passed")
        }
        _ => false,
    })
}

fn prioritize_evaluative_review_memory(working_memory: &mut [WorkingMemoryItem]) {
    working_memory.sort_by_key(|item| match item {
        WorkingMemoryItem::Text { label, .. } if label == "Cargo Check" => 0,
        WorkingMemoryItem::Text { label, .. } if label == "Cargo Clippy" => 1,
        WorkingMemoryItem::Text { label, .. } if label == "Cargo Test" => 2,
        WorkingMemoryItem::Text { label, .. } if label == "Rust Analyzer" => 3,
        WorkingMemoryItem::FileExcerpt(_) => 4,
        WorkingMemoryItem::DiffSummary(_) => 5,
        WorkingMemoryItem::Text { label, .. } if label == "Project Overview" => 6,
        WorkingMemoryItem::Text { .. } => 7,
    });
}

fn prioritize_edit_memory(working_memory: &mut [WorkingMemoryItem]) {
    working_memory.sort_by_key(|item| match item {
        WorkingMemoryItem::Text { label, .. } if label == "Likely Edit Targets" => 0,
        WorkingMemoryItem::Text { label, .. } if label == "Current Code Facts" => 1,
        WorkingMemoryItem::Text { label, .. } if label == "File Skeletons" => 2,
        WorkingMemoryItem::DiffSummary(_) => 3,
        WorkingMemoryItem::FileExcerpt(_) => 4,
        WorkingMemoryItem::Text { label, .. } if label == "Cargo Check" => 5,
        WorkingMemoryItem::Text { label, .. } if label == "Cargo Clippy" => 6,
        WorkingMemoryItem::Text { label, .. } if label == "Cargo Test" => 7,
        WorkingMemoryItem::Text { label, .. } if label == "Rust Analyzer" => 8,
        WorkingMemoryItem::Text { label, .. } if label == "Project Overview" => 9,
        WorkingMemoryItem::Text { .. } => 10,
    });
}

fn current_code_facts(working_memory: &[WorkingMemoryItem]) -> Option<String> {
    let excerpts = working_memory.iter().filter_map(|item| match item {
        WorkingMemoryItem::FileExcerpt(excerpt) => Some(excerpt),
        _ => None,
    });

    let mut structs = Vec::new();
    let mut enums = Vec::new();
    let mut functions = Vec::new();
    let mut fields = Vec::new();
    let mut env_vars = Vec::new();

    for excerpt in excerpts {
        for raw_line in excerpt.text.lines() {
            let line = raw_line.trim();

            if let Some(name) =
                parse_named_item(line, "pub struct ").or_else(|| parse_named_item(line, "struct "))
            {
                push_unique(&mut structs, name);
            }

            if let Some(name) =
                parse_named_item(line, "pub enum ").or_else(|| parse_named_item(line, "enum "))
            {
                push_unique(&mut enums, name);
            }

            if let Some(name) = parse_fn_name(line) {
                push_unique(&mut functions, name);
            }

            if let Some(field) = parse_field_name(line) {
                push_unique(&mut fields, field);
            }

            for env_var in parse_env_vars(line) {
                push_unique(&mut env_vars, env_var);
            }
        }
    }

    let mut sections = Vec::new();
    append_fact_section(&mut sections, "Structs already present", &structs);
    append_fact_section(&mut sections, "Enums already present", &enums);
    append_fact_section(&mut sections, "Functions already present", &functions);
    append_fact_section(&mut sections, "Fields already present", &fields);
    append_fact_section(
        &mut sections,
        "Environment variables already present",
        &env_vars,
    );

    if sections.is_empty() {
        None
    } else {
        Some(sections.join("\n"))
    }
}

/// Build a compact per-file skeleton (top-level pub enums with variants,
/// structs with field declarations, and fn signatures) for every file
/// referenced by a `FileExcerpt` in working memory. Lets the model see the
/// full shape of types it might modify without depending on the excerpt
/// window happening to include the struct's opening brace.
fn file_skeletons(
    working_memory: &[WorkingMemoryItem],
    project_root: &std::path::Path,
) -> Option<String> {
    let mut seen_paths: Vec<String> = Vec::new();
    for item in working_memory {
        if let WorkingMemoryItem::FileExcerpt(excerpt) = item
            && excerpt.path.ends_with(".rs")
            && !seen_paths.iter().any(|p| p == &excerpt.path)
        {
            seen_paths.push(excerpt.path.clone());
        }
    }
    if seen_paths.is_empty() {
        return None;
    }

    let mut sections = Vec::new();
    for relative in seen_paths.iter().take(6) {
        let full = project_root.join(relative);
        let Ok(content) = std::fs::read_to_string(&full) else {
            continue;
        };
        // Prefer the parser-backed skeleton (exact signatures incl. private items); fall
        // back to the line-scan heuristic when the file doesn't parse.
        let skeleton =
            render_file_skeleton_syn(&content).unwrap_or_else(|| render_file_skeleton(&content));
        if skeleton.trim().is_empty() {
            continue;
        }
        sections.push(format!("File: {relative}\n{skeleton}"));
    }

    if sections.is_empty() {
        None
    } else {
        Some(sections.join("\n\n"))
    }
}

/// Parser-backed file skeleton: exact source signatures of every top-level item
/// (structs, enums, unions, type aliases, fns, impls, traits) — public AND private —
/// sliced verbatim from the file so the model can copy anchors byte-for-byte. Returns
/// None when the file does not parse (e.g. mid-edit), so the caller falls back to the
/// line-scan skeleton.
fn render_file_skeleton_syn(content: &str) -> Option<String> {
    use syn::spanned::Spanned;

    let file = syn::parse_file(content).ok()?;
    let lines: Vec<&str> = content.lines().collect();

    // Inclusive 1-based line range (from proc-macro2 span-locations) → verbatim slice.
    let slice = |start_line: usize, end_line: usize| -> String {
        let start = start_line.saturating_sub(1);
        let end = end_line.min(lines.len());
        if start >= end {
            String::new()
        } else {
            lines[start..end].join("\n")
        }
    };
    // Earliest line of an item including its outer attributes (e.g. `#[derive(...)]`),
    // so the anchor text the model copies matches the file exactly.
    let start_with_attrs = |attrs: &[syn::Attribute], span: proc_macro2::Span| -> usize {
        attrs
            .iter()
            .map(|a| a.span().start().line)
            .min()
            .unwrap_or(span.start().line)
            .min(span.start().line)
    };

    let mut out: Vec<String> = Vec::new();
    for item in &file.items {
        match item {
            syn::Item::Struct(it) => {
                out.push(slice(start_with_attrs(&it.attrs, it.span()), it.span().end().line))
            }
            syn::Item::Enum(it) => {
                out.push(slice(start_with_attrs(&it.attrs, it.span()), it.span().end().line))
            }
            syn::Item::Union(it) => {
                out.push(slice(start_with_attrs(&it.attrs, it.span()), it.span().end().line))
            }
            syn::Item::Type(it) => {
                out.push(slice(start_with_attrs(&it.attrs, it.span()), it.span().end().line))
            }
            syn::Item::Fn(it) => {
                let sig = slice(it.sig.span().start().line, it.sig.span().end().line);
                out.push(format!("{} {{ ... }}", trim_sig(&sig)));
            }
            syn::Item::Impl(it) => {
                out.push(slice(it.span().start().line, it.span().start().line));
                for sub in &it.items {
                    if let syn::ImplItem::Fn(m) = sub {
                        let sig = slice(m.sig.span().start().line, m.sig.span().end().line);
                        out.push(format!("    {} {{ ... }}", trim_sig(&sig)));
                    }
                }
                out.push("}".to_string());
            }
            syn::Item::Trait(it) => {
                out.push(slice(it.span().start().line, it.span().start().line));
                for sub in &it.items {
                    if let syn::TraitItem::Fn(m) = sub {
                        let sig = slice(m.sig.span().start().line, m.sig.span().end().line);
                        out.push(format!("    {} {{ ... }}", trim_sig(&sig)));
                    }
                }
                out.push("}".to_string());
            }
            _ => {}
        }
        if out.len() > 400 {
            out.push("// ... (truncated)".to_string());
            break;
        }
    }

    if out.is_empty() {
        None
    } else {
        Some(out.join("\n"))
    }
}

/// Normalize a signature slice: drop a trailing `{` (body brace shares the sig's last
/// line) or `;` (bodyless trait method) so we can append a uniform ` { ... }`.
fn trim_sig(sig: &str) -> String {
    sig.trim()
        .trim_end_matches('{')
        .trim_end_matches(';')
        .trim_end()
        .to_string()
}

fn render_file_skeleton(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut out: Vec<String> = Vec::new();
    let mut depth: i32 = 0;
    let mut i = 0;

    while i < lines.len() {
        let raw = lines[i];
        let trimmed = raw.trim();

        if depth == 0 {
            // Capture top-level pub items.
            if let Some(name) = parse_named_item(trimmed, "pub struct ")
                .or_else(|| parse_named_item(trimmed, "pub(crate) struct "))
            {
                let (block, consumed) = collect_block(&lines, i);
                if has_brace(&block) {
                    let fields = extract_struct_fields(&block);
                    if fields.is_empty() {
                        out.push(format!("  pub struct {name} {{}}"));
                    } else {
                        out.push(format!("  pub struct {name} {{"));
                        for f in fields {
                            out.push(format!("    {f}"));
                        }
                        out.push("  }".to_string());
                    }
                } else {
                    // Tuple struct or unit struct on one line.
                    out.push(format!("  pub struct {name}"));
                }
                i += consumed;
                continue;
            }

            if let Some(name) = parse_named_item(trimmed, "pub enum ")
                .or_else(|| parse_named_item(trimmed, "pub(crate) enum "))
            {
                let (block, consumed) = collect_block(&lines, i);
                let variants = extract_enum_variants(&block);
                if variants.is_empty() {
                    out.push(format!("  pub enum {name} {{}}"));
                } else {
                    out.push(format!(
                        "  pub enum {name} {{ {} }}",
                        variants.join(", ")
                    ));
                }
                i += consumed;
                continue;
            }

        }

        // Capture pub fn signatures both at top level and inside `impl`
        // blocks (depth==1) so methods on a type show up in the skeleton.
        if depth <= 1
            && let Some(sig) = parse_pub_fn_signature(trimmed)
        {
            out.push(format!("  {sig}"));
        }

        // Track depth even for lines we didn't otherwise process so that
        // nested items inside `impl` / `mod` blocks are skipped.
        depth += brace_delta(raw);
        if depth < 0 {
            depth = 0;
        }
        i += 1;

        if out.len() > 200 {
            out.push("  ... (truncated)".to_string());
            break;
        }
    }

    out.join("\n")
}

fn brace_delta(line: &str) -> i32 {
    let mut delta: i32 = 0;
    let mut in_string = false;
    let mut prev = ' ';
    for c in line.chars() {
        if c == '"' && prev != '\\' {
            in_string = !in_string;
        } else if !in_string {
            if c == '{' {
                delta += 1;
            } else if c == '}' {
                delta -= 1;
            }
        }
        prev = c;
    }
    delta
}

fn collect_block(lines: &[&str], start: usize) -> (String, usize) {
    // Returns (joined block text, number of lines consumed starting at `start`).
    let first = lines[start];
    let mut depth = brace_delta(first);
    let mut joined = String::from(first);
    let mut consumed = 1usize;

    if depth == 0 && first.contains('{') {
        // single-line block like `struct Foo {}` - already balanced.
        return (joined, consumed);
    }

    if depth == 0 {
        // No opening brace on first line: scan ahead for it within a couple of lines.
        let mut probe = start + 1;
        while probe < lines.len() && probe - start < 4 {
            depth += brace_delta(lines[probe]);
            joined.push('\n');
            joined.push_str(lines[probe]);
            consumed += 1;
            if depth > 0 {
                break;
            }
            probe += 1;
        }
        if depth == 0 {
            return (joined, consumed);
        }
    }

    while depth > 0 && start + consumed < lines.len() {
        let next = lines[start + consumed];
        depth += brace_delta(next);
        joined.push('\n');
        joined.push_str(next);
        consumed += 1;
    }

    (joined, consumed)
}

fn has_brace(block: &str) -> bool {
    block.contains('{')
}

fn extract_struct_fields(block: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut depth: i32 = 0;
    for raw in block.lines() {
        let trimmed = raw.trim();
        let old_depth = depth;
        depth += brace_delta(raw);
        // Only consider lines at depth==1 (inside the struct body, not nested).
        if (old_depth == 1 || (old_depth == 0 && trimmed.contains('{') && depth >= 1))
            && let Some(field) = parse_struct_field_line(trimmed)
        {
            fields.push(field);
        }
    }
    fields
}

fn parse_struct_field_line(line: &str) -> Option<String> {
    if line.is_empty()
        || line.starts_with("//")
        || line.starts_with("#[")
        || line.starts_with("/*")
        || line.starts_with('}')
        || line.starts_with('{')
    {
        return None;
    }
    // Strip trailing comma and any inline comment.
    let body = line.split("//").next()?.trim_end_matches(',').trim();
    if !body.contains(':') {
        return None;
    }
    let starts_pub = body.starts_with("pub ");
    let stripped = body.strip_prefix("pub ").unwrap_or(body);
    let (name, ty) = stripped.split_once(':')?;
    let name = name.trim();
    let ty = ty.trim();
    if name.is_empty() || ty.is_empty() {
        return None;
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return None;
    }
    let prefix = if starts_pub { "pub " } else { "" };
    Some(format!("{prefix}{name}: {ty}"))
}

fn extract_enum_variants(block: &str) -> Vec<String> {
    let mut variants = Vec::new();
    let mut depth: i32 = 0;
    for raw in block.lines() {
        let trimmed = raw.trim();
        let old_depth = depth;
        depth += brace_delta(raw);
        if old_depth != 1 {
            continue;
        }
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("#[") {
            continue;
        }
        // Variant lines look like `Name,` or `Name(Inner),` or `Name { ... },`.
        let head = trimmed
            .split(|c: char| c == ',' || c == '(' || c == '{' || c.is_whitespace())
            .next()
            .unwrap_or("");
        if head.is_empty()
            || !head
                .chars()
                .next()
                .map(|c| c.is_ascii_uppercase() || c == '_')
                .unwrap_or(false)
            || !head
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            continue;
        }
        variants.push(head.to_string());
    }
    variants
}

fn parse_pub_fn_signature(line: &str) -> Option<String> {
    for prefix in [
        "pub async fn ",
        "pub(crate) async fn ",
        "pub fn ",
        "pub(crate) fn ",
    ] {
        if line.starts_with(prefix) {
            // Trim trailing `{` (body opener) or `;` from the signature.
            let head = line
                .split('{')
                .next()
                .unwrap_or(line)
                .trim_end_matches(';')
                .trim()
                .to_string();
            return Some(head);
        }
    }
    None
}

fn parse_named_item<'a>(line: &'a str, prefix: &str) -> Option<&'a str> {
    let remainder = line.strip_prefix(prefix)?;
    let name = remainder
        .split(|c: char| c.is_whitespace() || matches!(c, '{' | '(' | '<' | ':'))
        .next()?
        .trim();

    (!name.is_empty()).then_some(name)
}

fn parse_fn_name(line: &str) -> Option<&str> {
    for prefix in ["pub async fn ", "async fn ", "pub fn ", "fn "] {
        if let Some(remainder) = line.strip_prefix(prefix) {
            let name = remainder
                .split(|c: char| c.is_whitespace() || matches!(c, '(' | '<'))
                .next()?
                .trim();
            if !name.is_empty() {
                return Some(name);
            }
        }
    }

    None
}

fn parse_field_name(line: &str) -> Option<&str> {
    if line.starts_with("use ")
        || line.starts_with("pub use ")
        || line.starts_with("impl ")
        || line.starts_with("fn ")
        || line.starts_with("pub fn ")
        || line.starts_with("async fn ")
        || line.starts_with("pub async fn ")
        || line.starts_with('#')
    {
        return None;
    }

    let candidate = line
        .strip_prefix("pub ")
        .unwrap_or(line)
        .split_once(':')
        .map(|(name, _)| name.trim())?;

    if candidate.is_empty()
        || candidate.contains(' ')
        || !candidate
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return None;
    }

    Some(candidate)
}

fn parse_env_vars(line: &str) -> Vec<&str> {
    let mut vars = Vec::new();
    let mut remainder = line;

    while let Some(start) = remainder.find('"') {
        let after_quote = &remainder[start + 1..];
        let Some(end) = after_quote.find('"') else {
            break;
        };
        let candidate = &after_quote[..end];
        if is_env_var_name(candidate) {
            vars.push(candidate);
        }
        remainder = &after_quote[end + 1..];
    }

    vars
}

fn is_env_var_name(candidate: &str) -> bool {
    candidate.len() >= 3
        && candidate.contains('_')
        && candidate
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
}

fn push_unique(values: &mut Vec<String>, value: &str) {
    if !values.iter().any(|existing| existing == value) {
        values.push(value.to_string());
    }
}

fn append_fact_section(sections: &mut Vec<String>, label: &str, values: &[String]) {
    if values.is_empty() {
        return;
    }

    sections.push(format!("- {}: {}", label, values.join(", ")));
}

struct EditRunMetrics<'a> {
    model_name: &'a str,
    provider: &'a str,
    generation_ms: u128,
    turn_total_ms: u128,
    prompt_tokens: usize,
    response_chars: usize,
    response_tokens: usize,
    patches_parsed: usize,
    parse_errors: usize,
    edits_ok: usize,
    edits_blocked: usize,
    placeholder_hits: usize,
    retries_used: u32,
    max_retries: u32,
}

fn build_edit_run_summary(m: EditRunMetrics<'_>) -> String {
    format!(
        "--- Edit Run Summary ---\n\
         model:            {} ({})\n\
         generate_ms:      {} (total across {} attempt(s))\n\
         turn_total_ms:    {}\n\
         prompt_tokens~=:  {} (last attempt)\n\
         response_chars:   {}\n\
         response_tokens~: {}\n\
         patches_parsed:   {}\n\
         parse_errors:     {}\n\
         edits_ok:         {}\n\
         edits_blocked:    {}\n\
         placeholder_hits: {}\n\
         retries_used:     {}/{}",
        m.model_name,
        m.provider,
        m.generation_ms,
        m.retries_used + 1,
        m.turn_total_ms,
        m.prompt_tokens,
        m.response_chars,
        m.response_tokens,
        m.patches_parsed,
        m.parse_errors,
        m.edits_ok,
        m.edits_blocked,
        m.placeholder_hits,
        m.retries_used,
        m.max_retries,
    )
}

/// Heuristic quality of an edit attempt, used to keep the best result across retries
/// rather than emitting whatever the final (possibly degraded) iteration produced.
/// Applicable patches dominate; emitting *any* structured patch beats a format failure
/// (no patch envelope at all); blocked anchors and parse/verify errors subtract.
fn attempt_quality(patches: usize, ok: usize, blocked: usize, errors: usize) -> i64 {
    (patches as i64) * 50 + (ok as i64) * 100 - (blocked as i64) * 40 - (errors as i64) * 10
}

fn count_anchor_status(verified: &crate::patch::VerifiedPatches) -> (usize, usize) {
    let mut ok = 0usize;
    let mut blocked = 0usize;
    for patch in &verified.patches {
        match patch {
            crate::patch::VerifiedPatch::Modify { edits, .. } => {
                for edit in edits {
                    if edit.status.is_applicable() {
                        ok += 1;
                    } else {
                        blocked += 1;
                    }
                }
            }
            crate::patch::VerifiedPatch::Create {
                file_already_exists,
                ..
            } => {
                if *file_already_exists {
                    blocked += 1;
                } else {
                    ok += 1;
                }
            }
        }
    }
    (ok, blocked)
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

#[derive(Debug, Clone)]
pub struct OneShotSummary {
    pub mode: SessionMode,
    pub message: String,
    pub iterations: u32,
    pub max_retries: u32,
    pub patches_proposed: usize,
    pub patches_applicable: usize,
    pub anchor_failures: usize,
    pub elapsed_ms: u128,
}

#[derive(Debug, Clone)]
struct ContextualFollowup {
    intent: RustIntent,
    subject_query: String,
}

fn merge_query_context(
    query: &str,
    pending: Option<&PendingClarification>,
    followup: Option<&ContextualFollowup>,
) -> String {
    if let Some(pending) = pending {
        return format!(
            "{} (clarification: {})",
            pending.original_query,
            query.trim()
        );
    }

    if let Some(followup) = followup {
        return format!("{} about {}", query.trim(), followup.subject_query);
    }

    query.trim().to_string()
}

fn contextual_followup(
    query: &str,
    last_subject: Option<&SubjectAnchor>,
) -> Option<ContextualFollowup> {
    let q = query.trim().to_ascii_lowercase();
    let subject = last_subject?;

    if subject.mode != SessionMode::Ask {
        if subject.mode == SessionMode::Review
            && contains_any(
                &q,
                &[
                    "is that it",
                    "anything else",
                    "what else",
                    "go on",
                    "continue",
                    "more",
                ],
            )
        {
            return Some(ContextualFollowup {
                intent: subject.intent,
                subject_query: subject.query.clone(),
            });
        }
        return None;
    }

    let intent = if contains_any(
        &q,
        &[
            "show me an example",
            "can you show me",
            "example",
            "sample",
            "snippet",
        ],
    ) {
        RustIntent::ExampleLookup
    } else if contains_any(
        &q,
        &["explain more", "tell me more", "expand", "why", "which one"],
    ) {
        subject.intent
    } else {
        return None;
    };

    Some(ContextualFollowup {
        intent,
        subject_query: subject.query.clone(),
    })
}

fn contains_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}

fn trimmed_history(
    history: &[crate::session::ConversationTurn],
    max_turns: usize,
    max_chars: usize,
) -> String {
    let selected = history
        .iter()
        .rev()
        .take(max_turns)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|turn| format!("User: {}\nAssistant: {}", turn.query, turn.response))
        .collect::<Vec<_>>();

    let joined = selected.join("\n");
    let chars = joined.chars().count();
    if chars <= max_chars {
        return joined;
    }

    let truncated = joined
        .chars()
        .skip(chars.saturating_sub(max_chars))
        .collect::<String>();

    format!("...[truncated earlier conversation]\n{truncated}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attempt_quality_prefers_applicable_patch_over_format_failure() {
        // (patches, ok, blocked, errors). An applicable patch with a compile error still
        // beats a parse failure with no patches — the regression that made retries emit a
        // worse result.
        let applicable_with_compile_error = attempt_quality(1, 1, 0, 1); // iter 0
        let format_failure = attempt_quality(0, 0, 0, 1); // iter 2 (malformed)
        assert!(applicable_with_compile_error > format_failure);
    }

    #[test]
    fn attempt_quality_prefers_parsed_patch_over_no_patch() {
        // A parsed-but-anchor-blocked patch is closer to a usable edit than an empty
        // format failure (the Strand-run case), so it must score higher.
        let blocked_patch = attempt_quality(1, 0, 1, 0);
        let format_failure = attempt_quality(0, 0, 0, 0);
        assert!(blocked_patch > format_failure);
    }

    #[test]
    fn attempt_quality_penalizes_blocked_and_errors() {
        assert!(attempt_quality(1, 2, 0, 0) > attempt_quality(1, 2, 1, 0));
        assert!(attempt_quality(1, 1, 0, 0) > attempt_quality(1, 1, 0, 3));
        assert_eq!(attempt_quality(0, 0, 0, 0), 0);
    }

    fn subject(mode: SessionMode, intent: RustIntent, query: &str) -> SubjectAnchor {
        SubjectAnchor {
            mode,
            intent,
            query: query.to_string(),
        }
    }

    #[test]
    fn merge_query_context_uses_pending_clarification() {
        let pending = PendingClarification {
            intent: RustIntent::CrateSelection,
            original_query: "what crate should I use for retries?".to_string(),
        };

        let merged = merge_query_context("http", Some(&pending), None);
        assert_eq!(
            merged,
            "what crate should I use for retries? (clarification: http)"
        );
    }

    #[test]
    fn contextual_followup_reuses_subject_for_examples() {
        let followup = contextual_followup(
            "can you show me an example?",
            Some(&subject(
                SessionMode::Ask,
                RustIntent::CrateSelection,
                "what crate should I use for retries? (clarification: http)",
            )),
        )
        .expect("expected follow-up");

        assert_eq!(followup.intent, RustIntent::ExampleLookup);
        assert!(followup.subject_query.contains("clarification: http"));
    }

    #[test]
    fn merge_query_context_uses_followup_subject() {
        let followup = ContextualFollowup {
            intent: RustIntent::ExampleLookup,
            subject_query: "what crate should I use for retries? (clarification: http)".to_string(),
        };

        let merged = merge_query_context("can you show me an example?", None, Some(&followup));
        assert_eq!(
            merged,
            "can you show me an example? about what crate should I use for retries? (clarification: http)"
        );
    }

    #[test]
    fn non_ask_subject_does_not_trigger_contextual_followup() {
        let followup = contextual_followup(
            "can you show me an example?",
            Some(&subject(
                SessionMode::Review,
                RustIntent::EvaluativeReview,
                "what is wrong with this project?",
            )),
        );

        assert!(followup.is_none());
    }

    #[test]
    fn review_followup_reuses_prior_review_subject() {
        let followup = contextual_followup(
            "is that it?",
            Some(&subject(
                SessionMode::Review,
                RustIntent::EvaluativeReview,
                "what is wrong with this project?",
            )),
        )
        .expect("expected review follow-up");

        assert_eq!(followup.intent, RustIntent::EvaluativeReview);
        assert_eq!(followup.subject_query, "what is wrong with this project?");
    }

    #[test]
    fn trimmed_history_keeps_recent_turns_with_budget() {
        let history = vec![
            crate::session::ConversationTurn {
                query: "one".to_string(),
                response: "alpha".to_string(),
            },
            crate::session::ConversationTurn {
                query: "two".to_string(),
                response: "beta".to_string(),
            },
            crate::session::ConversationTurn {
                query: "three".to_string(),
                response: "gamma".to_string(),
            },
        ];

        let trimmed = trimmed_history(&history, 2, 80);
        assert!(!trimmed.contains("one"));
        assert!(trimmed.contains("two"));
        assert!(trimmed.contains("three"));
    }

    #[test]
    fn vague_edit_requests_skip_rust_analyzer() {
        assert!(!should_run_rust_analyzer(
            SessionMode::Edit,
            RustIntent::FeatureImplementation
        ));
        assert!(!should_run_rust_analyzer(
            SessionMode::Edit,
            RustIntent::CompileFix
        ));
        assert!(should_run_rust_analyzer(
            SessionMode::Edit,
            RustIntent::Refactor
        ));
    }

    #[test]
    fn file_skeletons_renders_struct_fields_enum_variants_and_pub_fns() {
        let tmp = std::env::temp_dir().join(format!(
            "rustopedia_skel_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&tmp).unwrap();
        let file_rel = "src/sample.rs";
        let file_full = tmp.join(file_rel);
        std::fs::create_dir_all(file_full.parent().unwrap()).unwrap();
        std::fs::write(
            &file_full,
            r#"
pub enum LlmProvider {
    Ollama,
    OpenRouter,
}

pub struct AppConfig {
    pub llm_provider: LlmProvider,
    pub openrouter_api_key: Option<String>,
    edit_max_retries: u32,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            llm_provider: LlmProvider::Ollama,
            openrouter_api_key: None,
            edit_max_retries: 2,
        }
    }
    pub async fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
"#,
        )
        .unwrap();

        let working_memory = vec![WorkingMemoryItem::FileExcerpt(crate::memory::FileExcerpt {
            path: file_rel.to_string(),
            start_line: 1,
            end_line: 5,
            text: "irrelevant; the extractor re-reads the real file".to_string(),
        })];

        let skeleton =
            file_skeletons(&working_memory, &tmp).expect("expected a skeleton block");
        assert!(skeleton.contains("File: src/sample.rs"));
        // The parser-backed skeleton emits exact source slices.
        assert!(skeleton.contains("pub enum LlmProvider {"));
        assert!(skeleton.contains("Ollama,"));
        assert!(skeleton.contains("OpenRouter,"));
        assert!(skeleton.contains("pub struct AppConfig {"));
        assert!(skeleton.contains("pub llm_provider: LlmProvider"));
        assert!(skeleton.contains("pub openrouter_api_key: Option<String>"));
        assert!(skeleton.contains("edit_max_retries: u32"));
        assert!(skeleton.contains("pub fn from_env() -> Self"));
        assert!(skeleton.contains("pub async fn validate(&self) -> Result<(), String>"));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn syn_skeleton_includes_private_items_with_exact_signatures() {
        // The Strand anchor-fidelity case: a PRIVATE struct with a derive attribute. The
        // old line-scan skeleton omitted it entirely (pub-only); the model then guessed
        // `pub struct` and the anchor failed.
        let src = "#[derive(Deserialize)]\nstruct OpenRouterChatResponse {\n    choices: Vec<OpenRouterChoice>,\n}\n\npub fn run(x: u32) -> bool {\n    x > 0\n}\n";
        let skel = render_file_skeleton_syn(src).expect("should parse");

        assert!(skel.contains("#[derive(Deserialize)]"));
        assert!(skel.contains("struct OpenRouterChatResponse {"));
        assert!(skel.contains("    choices: Vec<OpenRouterChoice>,"));
        // Private struct must NOT be rewritten as `pub` — that was the mismatch.
        assert!(!skel.contains("pub struct OpenRouterChatResponse"));
        assert!(skel.contains("pub fn run(x: u32) -> bool { ... }"));
    }

    #[test]
    fn syn_skeleton_returns_none_on_parse_error() {
        // Mid-edit / unparseable file → None so file_skeletons falls back to line-scan.
        assert!(render_file_skeleton_syn("fn broken( { struct").is_none());
    }

    #[test]
    fn current_code_facts_extracts_existing_symbols_and_env_vars() {
        let working_memory = vec![
            WorkingMemoryItem::Text {
                label: "Likely Edit Targets".to_string(),
                content: "src/config.rs".to_string(),
            },
            WorkingMemoryItem::FileExcerpt(crate::memory::FileExcerpt {
                path: "src/config.rs".to_string(),
                start_line: 1,
                end_line: 14,
                text: r#"
pub enum LlmProvider {
    Ollama,
    OpenRouter,
}

pub struct AppConfig {
    pub openrouter_base_url: String,
    pub openrouter_api_key: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        env::var("RUSTOPEDIA_OPENROUTER_BASE_URL").ok();
"#
                .trim()
                .to_string(),
            }),
            WorkingMemoryItem::FileExcerpt(crate::memory::FileExcerpt {
                path: "src/llm.rs".to_string(),
                start_line: 30,
                end_line: 36,
                text: r#"
async fn generate_openrouter(&self, model: &str, prompt: &str) -> Result<String> {
    let url = format!(
        "{}/api/v1/chat/completions",
        self.config.openrouter_base_url.trim_end_matches('/')
    );
"#
                .trim()
                .to_string(),
            }),
        ];

        let facts = current_code_facts(&working_memory).expect("expected facts");

        assert!(facts.contains("Structs already present: AppConfig"));
        assert!(facts.contains("Enums already present: LlmProvider"));
        assert!(facts.contains("Functions already present: from_env, generate_openrouter"));
        assert!(facts.contains("Fields already present: openrouter_base_url, openrouter_api_key"));
        assert!(
            facts.contains("Environment variables already present: RUSTOPEDIA_OPENROUTER_BASE_URL")
        );
    }

    #[test]
    fn prioritize_edit_memory_places_code_facts_near_top() {
        let mut items = vec![
            WorkingMemoryItem::Text {
                label: "Cargo Check".to_string(),
                content: "ok".to_string(),
            },
            WorkingMemoryItem::Text {
                label: "Current Code Facts".to_string(),
                content: "facts".to_string(),
            },
            WorkingMemoryItem::Text {
                label: "Likely Edit Targets".to_string(),
                content: "targets".to_string(),
            },
        ];

        prioritize_edit_memory(&mut items);

        assert!(matches!(
            &items[0],
            WorkingMemoryItem::Text { label, .. } if label == "Likely Edit Targets"
        ));
        assert!(matches!(
            &items[1],
            WorkingMemoryItem::Text { label, .. } if label == "Current Code Facts"
        ));
    }
}

