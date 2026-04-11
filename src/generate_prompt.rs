use serde_json::Value;

use crate::planner::PlanStep;
use crate::session::SessionMode;

pub fn build_prompt(
    mode: SessionMode,
    past: &str,
    context: &str,
    query: &str,
    plan: &[PlanStep],
) -> String {
    let mode_instruction = match mode {
        SessionMode::Ask => {
            "You are a helpful Rust programming assistant. Answer directly and use provided context when it helps."
        }
        SessionMode::Review => {
            "You are reviewing or explaining Rust code. If the user is asking for evaluation, findings and risks come first. If the user is asking for understanding, give a clear walkthrough grounded in the available context."
        }
        SessionMode::Edit => {
            "You are preparing a Rust code change workflow. Focus on the intended edit, affected areas, and verification steps. If the request lacks enough repo context, say what is missing."
        }
    };

    let plan_summary = if plan.is_empty() {
        "No tools were selected.".to_string()
    } else {
        plan.iter()
            .map(|step| format!("- {}: {}", step.tool, step.input))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        "{mode_instruction}

### Current Mode:
{}

### Planned Steps:
{}

### Past Conversation:
{}

### Context:
{}

### User Question:
{}

### Answer:",
        mode.as_str(),
        plan_summary,
        past.trim(),
        context.trim(),
        query.trim()
    )
}

pub fn format_agent_output_for_llm(tool_name: &str, agent_info: &Value) -> String {
    let mut context_sections = Vec::new();

    context_sections.push(format!("From {}:", tool_name));

    if agent_info.get("results").is_some() {
        context_sections.push("Crate Info:".to_string());
        for item in agent_info["results"].as_array().unwrap_or(&vec![]) {
            let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let desc = item
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let ver = item.get("version").and_then(|v| v.as_str()).unwrap_or("");
            let homepage = item.get("homepage").and_then(|v| v.as_str()).unwrap_or("");
            context_sections.push(format!(
                "- {} (v{}): {}
  {}",
                name, ver, desc, homepage
            ));
        }
    }

    if agent_info.get("docs").is_some() {
        context_sections.push("Documentation Snippets:".to_string());
        for doc in agent_info["docs"].as_array().unwrap_or(&vec![]) {
            let text = doc.get("text").and_then(|v| v.as_str()).unwrap_or("");
            context_sections.push(format!("> {}", text));
        }
    }

    // If no specific sections were found, just include the raw JSON string for debugging/fallback
    if context_sections.len() == 1 { // Only "From {tool_name}:" was added
        context_sections.push(format!("Raw output: {}", agent_info));
    }

    context_sections.join("\n")
}
