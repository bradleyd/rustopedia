use anyhow::{Context, Result, anyhow};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PlanStep {
    pub tool: String,
    pub input: String,
}

pub fn generate_tool_plan_prompt(question: &str) -> String {
    format!(
        "You are a Rust assistant that chooses which tools to run for answering a question.\nAvailable tools:\n- crate_agent: finds crates on crates.io (use for finding crate names or features)\n- github_agent: finds example Rust code on GitHub (use for vague questions or tutorials)\n- docs_agent: fetches Rust stdlib or crate docs from docs.rs (use for specific functions or stdlib modules)\n- none: for questions that don't need a tool and can be answered from general knowledge\n\nReturn ONLY valid JSON.\nSchema: [{{\"tool\":\"crate_agent|github_agent|docs_agent|none\",\"input\":\"string\"}}]\nQuestion: {}\nJSON:",
        question
    )
}

fn extract_first_json_array(text: &str) -> Option<&str> {
    let start = text.find('[')?;
    let end = text.rfind(']')?;
    if end < start {
        return None;
    }
    Some(&text[start..=end])
}

pub async fn generate_plan(
    question: &str,
    planner_model_name: &str,
    llm_client: &crate::llm::LlmClient,
) -> Result<Vec<PlanStep>> {
    let prompt = generate_tool_plan_prompt(question);
    let text = llm_client
        .generate(planner_model_name, &prompt)
        .await
        .context("failed to run planner model")?;

    let json_block = extract_first_json_array(&text).ok_or_else(|| {
        anyhow!(
            "planner returned no JSON array; output head: {}",
            text.chars().take(200).collect::<String>()
        )
    })?;

    let mut parsed: Vec<PlanStep> =
        serde_json::from_str(json_block).context("failed to parse planner JSON")?;

    parsed.retain(|step| step.tool != "none" && !step.tool.is_empty());
    Ok(parsed)
}
