use crate::intents::RustIntent;
use crate::session::SessionMode;

#[derive(Debug, Clone)]
pub struct PlanStep {
    pub tool: String,
    pub input: String,
}

pub fn generate_plan(question: &str, mode: SessionMode, intent: RustIntent) -> Vec<PlanStep> {
    match mode {
        SessionMode::Ask => generate_ask_plan(question, intent),
        SessionMode::Review | SessionMode::Edit => Vec::new(),
    }
}

fn generate_ask_plan(question: &str, intent: RustIntent) -> Vec<PlanStep> {
    let tool = match intent {
        RustIntent::CrateSelection => Some("crate_agent"),
        RustIntent::ApiLookup => Some("docs_agent"),
        RustIntent::ExampleLookup => Some("github_agent"),
        _ => None,
    };

    tool.map_or_else(Vec::new, |tool| {
        vec![PlanStep {
            tool: tool.to_string(),
            input: question.trim().to_string(),
        }]
    })
}
