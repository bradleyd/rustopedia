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
        SessionMode::Review => review_instruction(query),
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

fn review_instruction(query: &str) -> &'static str {
    match classify_review_intent(query) {
        ReviewIntent::ProjectPurpose => {
            "You are explaining a local Rust project. Answer the user’s question directly in 1-2 sentences first. Focus on the project’s purpose and current direction, derived primarily from code structure and runtime behavior. Use README details only to confirm or refine what the code already shows. Do not enumerate every file unless the user asks for architecture details. Do not add generic risks or boilerplate."
        }
        ReviewIntent::ArchitectureSummary => {
            "You are summarizing the architecture of a local Rust project. Start with a short top-level summary, then list the main architectural pieces and their responsibilities. Group related files into subsystems instead of enumerating every file. Do not add generic risks unless the user explicitly asks for review or evaluation."
        }
        ReviewIntent::EvaluativeReview => {
            "You are reviewing Rust code or a Rust project. Findings and risks come first, ordered by severity. Be concrete, grounded in the available context, and avoid generic advice."
        }
        ReviewIntent::CodePathExplanation => {
            "You are explaining how a local Rust code path works. Start with the high-level flow, then name the key modules or files involved. Keep the explanation grounded in code-derived context and avoid unrelated architectural commentary."
        }
        ReviewIntent::GeneralExplanation => {
            "You are explaining Rust code or a Rust project. Answer directly first, then expand only if useful. Use code-derived context before README-style summaries. Do not add generic risks or boilerplate unless the user asked for evaluation."
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReviewIntent {
    ProjectPurpose,
    ArchitectureSummary,
    EvaluativeReview,
    CodePathExplanation,
    GeneralExplanation,
}

fn classify_review_intent(query: &str) -> ReviewIntent {
    let q = query.to_ascii_lowercase();

    if q.contains("what does this project do")
        || q.contains("what is this project")
        || q.contains("what problem is this project trying to solve")
        || q.contains("what problem does this project solve")
    {
        ReviewIntent::ProjectPurpose
    } else if q.contains("architectural pieces")
        || q.contains("architecture")
        || q.contains("main modules")
        || q.contains("responsibilities")
    {
        ReviewIntent::ArchitectureSummary
    } else if q.contains("review this")
        || q.contains("what is wrong")
        || q.contains("risks")
        || q.contains("bugs")
        || q.contains("weak spots")
        || q.contains("brittle")
    {
        ReviewIntent::EvaluativeReview
    } else if q.contains("trace")
        || q.contains("flow")
        || q.contains("path")
        || q.contains("how does this work")
    {
        ReviewIntent::CodePathExplanation
    } else {
        ReviewIntent::GeneralExplanation
    }
}

pub fn review_intent(query: &str) -> &'static str {
    match classify_review_intent(query) {
        ReviewIntent::ProjectPurpose => "project_purpose",
        ReviewIntent::ArchitectureSummary => "architecture_summary",
        ReviewIntent::EvaluativeReview => "evaluative_review",
        ReviewIntent::CodePathExplanation => "code_path_explanation",
        ReviewIntent::GeneralExplanation => "general_explanation",
    }
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
