use anyhow::{Context, Result};
use serde_json::Value;
use std::io::{self, Write};

mod config;
mod generate_prompt;
mod llm;
mod planner;
mod router_llm;
mod tools;

use crate::config::AppConfig;
use crate::generate_prompt::format_agent_output_for_llm;
use planner::generate_plan;
use prettyprint::PrettyPrinter;

async fn run_agent(agent_type: &str, query: &str) -> Result<Option<String>> {
    match agent_type {
        "crate_agent" => tools::crate_search::search_crates(query)
            .await
            .map(|json| Some(serde_json::to_string_pretty(&json).unwrap_or_default()))
            .map_err(|e| anyhow::anyhow!("crate_agent request failed: {e}")),
        "docs_agent" => tools::docs::search_docs(query)
            .await
            .map(|json| Some(serde_json::to_string_pretty(&json).unwrap_or_default()))
            .map_err(|e| anyhow::anyhow!("docs_agent request failed: {e}")),
        "github_agent" => tools::github::search_github(query)
            .await
            .map(|json| Some(serde_json::to_string_pretty(&json).unwrap_or_default()))
            .map_err(|e| anyhow::anyhow!("github_agent request failed: {e}")),
        _ => Ok(None),
    }
}

fn generate_prompt(past: &str, context: &str, query: &str) -> String {
    format!(
        "You are a helpful Rust programming assistant. \
You may use your own knowledge and the provided documentation to answer user questions. \
If the documentation context is helpful, you may cite it. \
Use prior conversation history to maintain coherence when needed.

### Past conversation:
{}

### Context:
{}

### User question:
{}

### Answer:",
        past.trim(),
        context.trim(),
        query.trim()
    )
}

async fn call_rag(query: &str, collection: &str, top_k: usize) -> Result<String> {
    tools::qdrant_client::query_qdrant_with_text(collection, query, top_k)
        .await
        .map_err(|e| anyhow::anyhow!("Qdrant query failed for collection '{collection}': {e}"))
}

async fn call_local_llm(
    prompt: &str,
    config: &AppConfig,
    llm_client: &crate::llm::LlmClient,
) -> Result<String> {
    llm_client
        .generate(&config.model_name, prompt)
        .await
        .context("failed to generate final response")
}

#[tokio::main]
async fn main() {
    let mut history: Vec<(String, String)> = Vec::new();
    let config = AppConfig::from_env();
    if let Err(e) = config.validate() {
        eprintln!("❌ Invalid configuration: {e}");
        return;
    }
    let llm_client = crate::llm::LlmClient::new(config.clone());

    println!("Rust LLM Chat Assistant (type 'exit' to quit)");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let query = input.trim();
        if query == "exit" || query == "quit" {
            break;
        }

        let mut context_chunks = Vec::new();

        println!("Planning which tools to use...");
        let plan = match generate_plan(query, &config.planner_model_name, &llm_client).await {
            Ok(plan) => plan,
            Err(e) => {
                eprintln!("⚠️ Planner failed: {e}");
                Vec::new()
            }
        };
        println!("Plan {:?}", plan);

        for step in &plan {
            println!("Running agent: {} with input: {}", step.tool, step.input);
            match run_agent(&step.tool, &step.input).await {
                Ok(Some(response)) => match serde_json::from_str::<Value>(&response) {
                    Ok(json_value) => {
                        context_chunks.push(format_agent_output_for_llm(&step.tool, &json_value))
                    }
                    Err(_) => context_chunks.push(format!("From {}: {}", step.tool, response.trim())),
                },
                Ok(None) => {
                    eprintln!("⚠️ Unknown agent in plan: {}", step.tool);
                }
                Err(e) => {
                    eprintln!("⚠️ Agent failed ({}): {e}", step.tool);
                }
            }
        }

        println!("Getting RAG information");
        let mut rag_collections = std::collections::HashSet::new();
        for step in &plan {
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

        if rag_collections.is_empty() {
            rag_collections.insert("rust-docs");
        }

        for collection in rag_collections {
            match call_rag(query, collection, config.rag_top_k).await {
                Ok(rag) if !rag.trim().is_empty() => {
                    context_chunks.push(format!("From memory ({}): {}", collection, rag.trim()));
                }
                Ok(_) => {
                    eprintln!("⚠️ Empty RAG response for collection '{}'.", collection);
                }
                Err(e) => {
                    eprintln!("⚠️ {e}");
                }
            }
        }

        let past = history
            .iter()
            .map(|(q, a)| format!("User: {}\nAssistant: {}", q, a))
            .collect::<Vec<String>>()
            .join("\n");

        let full_context = context_chunks.join("\n\n");
        let prompt = generate_prompt(&past, &full_context, query);

        let response = match call_local_llm(&prompt, &config, &llm_client).await {
            Ok(response) => response,
            Err(e) => {
                eprintln!("❌ LLM call failed: {e}");
                continue;
            }
        };

        let printer = PrettyPrinter::default().language("rust").build();
        match printer {
            Ok(tprint) => {
                if tprint.string(&response).is_err() {
                    println!("\n{}\n", response.trim())
                }
            }
            Err(_) => println!("\n{}\n", response.trim()),
        }

        history.push((query.to_string(), response.trim().to_string()));
    }
}
