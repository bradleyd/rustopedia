use core::str;
use serde_json::Value;
use std::io::{self, Write};
use std::process::Command;
use urlencoding::encode;

mod planner;
mod router_llm;
mod tools;
use crate::generate_prompt::format_agent_output_for_llm;
use planner::generate_plan;
use prettyprint::PrettyPrinter;
use reqwest::Client;

mod generate_prompt;

async fn run_agent(agent_type: &str, query: &str) -> Option<String> {
    let result = match agent_type {
        "crate_agent" => {
            match tools::crate_search::search_crates(query).await {
                Ok(json) => Some(serde_json::to_string_pretty(&json).unwrap_or_default()),
                Err(_) => None,
            }
        }
        "docs_agent" => {
            match tools::docs::search_docs(query).await {
                Ok(json) => Some(serde_json::to_string_pretty(&json).unwrap_or_default()),
                Err(_) => None,
            }
        }
        "github_agent" => {
            match tools::github::search_github(query).await {
                Ok(json) => Some(serde_json::to_string_pretty(&json).unwrap_or_default()),
                Err(_) => None,
            }
        }
        _ => None,
    };
    result
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
async fn call_rag(query: &str, collection: &str) -> Option<String> {
    let client = Client::new();
    // Encode the query
    let encoded_query = encode(query);

    // "http://localhost:8000/query?collection=rust-docs&query=VecDequeu::new()&n_results=10"
    let url = format!(
        "http://localhost:8000/query?collection={}&query={}&n_results=10",
        collection, encoded_query
    );

    match client.get(&url).send().await {
        Ok(response) => match response.json::<Value>().await {
            Ok(json) => {
                let context = match json.get("docs") {
                    Some(Value::Array(docs)) => docs
                        .iter()
                        .filter_map(|doc| doc.get("content")) // get content field from each doc
                        .filter_map(|content| content.as_str())
                        .collect::<Vec<_>>()
                        .join("\n\n"),
                    _ => String::from("No docs found"),
                };
                println!("Rag response empty? {:?}", context.is_empty());
                Some(context)
            }
            Err(e) => {
                eprintln!("❌ Failed to parse JSON: {}", e);
                None
            }
        },
        Err(e) => {
            eprintln!("❌ HTTP request failed: {}", e);
            None
        }
    }
}

async fn call_local_llm(prompt: &str) -> String {
    let prompt_owned = prompt.to_string();
    tokio::task::spawn_blocking(move || {
        let mut child = Command::new("ollama")
            .args(["run", "openhermes"])
            //.args(["run", "deepseek-r1:8b"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to start LLM");

        {
            let stdin = child.stdin.as_mut().expect("Failed to open stdin");
            stdin.write_all(prompt_owned.as_bytes()).unwrap();
        }

        let output = child.wait_with_output().expect("Failed to read output");
        String::from_utf8_lossy(&output.stdout).to_string()
    }).await.unwrap_or_default()
}

#[tokio::main]
async fn main() {
    let mut history: Vec<(String, String)> = Vec::new();

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

        // 1. Get a plan from LLM
        println!("Planning which tools to use...");
        let plan = generate_plan(query).await;
        println!("Plan {:?}", plan);

        // TODO save tool and pass that to call_rag() to descide which collection to use
        // match vec contains {
        // crate_agent => crates
        // docs_agent => rust-docs,
        // github_agent => rust-book,
        // }
        // push to a set
        // This is because chromadb has different doc setup in the db and the url needs to reflect
        // which one to call via http.
        //
        // 2. Run each agent in the plan if a plan exists
        if !plan.is_empty() {
            for (tool, tool_input) in &plan {
                println!("Running agent: {} with input: {}", tool, tool_input);
                
                if let Some(response) = run_agent(tool, tool_input).await {
                    // Attempt to parse the response as JSON
                    match serde_json::from_str::<Value>(&response) {
                        Ok(json_value) => {
                            // If successful, format it using the new function
                            context_chunks.push(format_agent_output_for_llm(tool, &json_value));
                        }
                        Err(_) => {
                            // If not JSON, or parsing fails, just push the raw string
                            context_chunks.push(format!("From {}: {}", tool, response.trim()));
                        }
                    }
                }
            }
        }

        // 3. Always run RAG as well
        println!("Getting RAG information");
        let mut rag_collections = std::collections::HashSet::new();
        for (tool, _) in &plan {
            match tool.as_str() {
                "crate_agent" => { rag_collections.insert("crates"); },
                "docs_agent" => { rag_collections.insert("rust-docs"); },
                "github_agent" => { rag_collections.insert("rust-book"); },
                _ => { /* Do nothing for unknown tools */ }
            }
        }
        // Always include rust-docs by default if no specific agent suggested a collection
        if rag_collections.is_empty() {
            rag_collections.insert("rust-docs");
        }

        for collection in rag_collections {
            if let Some(rag) = call_rag(query, collection).await {
                context_chunks.push(format!("From memory ({}): {}", collection, rag.trim()));
            }
        }
        //        println!("Memory context retrieved. {:?}", context_chunks);

        // 4. Build conversation memory
        let past = history
            .iter()
            .map(|(q, a)| {
                format!(
                    "User: {}
Assistant: {}",
                    q, a
                )
            })
            .collect::<Vec<String>>()
            .join(
                "
",
            );
        // 5. Build and send prompt
        let full_context = context_chunks.join(
            "

",
        );

        let prompt = generate_prompt(&past, &full_context, query);
        //println!("DEBUG PROMPT: {:?}", prompt);
        let response = call_local_llm(&prompt).await;
        let printer = PrettyPrinter::default().language("rust").build();
        match printer {
            Ok(tprint) => {
                if tprint.string(&response).is_err() {
                    println!(
                        "
{}
",
                        response.trim()
                    )
                }
            }
            Err(_) => println!(
                "
{}
",
                response.trim()
            ),
        }

        history.push((query.to_string(), response.trim().to_string()));
    }
}
