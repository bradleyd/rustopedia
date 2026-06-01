use crate::session::{ConversationTurn, SessionMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RustIntent {
    GeneralRustQuestion,
    LocalWorkspaceQuestion,
    CrateSelection,
    ApiLookup,
    ExampleLookup,
    ProjectExplanation,
    ArchitectureSummary,
    CodePathExplanation,
    EvaluativeReview,
    CompileFix,
    ClippyFix,
    TestFix,
    TestCreation,
    FeatureImplementation,
    Refactor,
}

impl RustIntent {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GeneralRustQuestion => "general_rust_question",
            Self::LocalWorkspaceQuestion => "local_workspace_question",
            Self::CrateSelection => "crate_selection",
            Self::ApiLookup => "api_lookup",
            Self::ExampleLookup => "example_lookup",
            Self::ProjectExplanation => "project_explanation",
            Self::ArchitectureSummary => "architecture_summary",
            Self::CodePathExplanation => "code_path_explanation",
            Self::EvaluativeReview => "evaluative_review",
            Self::CompileFix => "compile_fix",
            Self::ClippyFix => "clippy_fix",
            Self::TestFix => "test_fix",
            Self::TestCreation => "test_creation",
            Self::FeatureImplementation => "feature_implementation",
            Self::Refactor => "refactor",
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntentScore {
    pub intent: RustIntent,
    pub score: i32,
}

pub fn infer_initial_intent(
    query: &str,
    mode: SessionMode,
    history: &[ConversationTurn],
) -> RustIntent {
    let mut scores = score_intents(query, mode, history);
    scores.sort_by(|a, b| b.score.cmp(&a.score));
    scores
        .into_iter()
        .next()
        .map(|score| score.intent)
        .unwrap_or(default_intent_for_mode(mode))
}

pub fn clarification_prompt(
    query: &str,
    mode: SessionMode,
    intent: RustIntent,
) -> Option<&'static str> {
    match (mode, intent) {
        (SessionMode::Ask, RustIntent::CrateSelection)
            if crate_selection_needs_clarification(query) =>
        {
            Some(
                "Is this for Tokio async code, generic retry/backoff around any operation, or HTTP request retries?",
            )
        }
        _ => None,
    }
}

pub fn score_intents(
    query: &str,
    mode: SessionMode,
    history: &[ConversationTurn],
) -> Vec<IntentScore> {
    let q = query.to_ascii_lowercase();
    let last_query = history
        .last()
        .map(|turn| turn.query.to_ascii_lowercase())
        .unwrap_or_default();

    let mut scores = match mode {
        SessionMode::Ask => vec![
            IntentScore {
                intent: RustIntent::GeneralRustQuestion,
                score: 10,
            },
            IntentScore {
                intent: RustIntent::LocalWorkspaceQuestion,
                score: 0,
            },
            IntentScore {
                intent: RustIntent::CrateSelection,
                score: 0,
            },
            IntentScore {
                intent: RustIntent::ApiLookup,
                score: 0,
            },
            IntentScore {
                intent: RustIntent::ExampleLookup,
                score: 0,
            },
        ],
        SessionMode::Review => vec![
            IntentScore {
                intent: RustIntent::ProjectExplanation,
                score: 5,
            },
            IntentScore {
                intent: RustIntent::ArchitectureSummary,
                score: 0,
            },
            IntentScore {
                intent: RustIntent::CodePathExplanation,
                score: 0,
            },
            IntentScore {
                intent: RustIntent::EvaluativeReview,
                score: 0,
            },
        ],
        SessionMode::Edit => vec![
            IntentScore {
                intent: RustIntent::FeatureImplementation,
                score: 5,
            },
            IntentScore {
                intent: RustIntent::CompileFix,
                score: 0,
            },
            IntentScore {
                intent: RustIntent::ClippyFix,
                score: 0,
            },
            IntentScore {
                intent: RustIntent::TestFix,
                score: 0,
            },
            IntentScore {
                intent: RustIntent::TestCreation,
                score: 0,
            },
            IntentScore {
                intent: RustIntent::Refactor,
                score: 0,
            },
        ],
    };

    boost_if(
        &mut scores,
        RustIntent::LocalWorkspaceQuestion,
        contains_any(
            &q,
            &[
                "this project",
                "this repo",
                "this repository",
                "this codebase",
                "this workspace",
                "our code",
                "our project",
                "local code",
                "current code",
                ".rs",
                "module",
                "runtime.rs",
                "config.rs",
                "main.rs",
                "session.rs",
            ],
        ),
        25,
    );
    boost_if(
        &mut scores,
        RustIntent::ProjectExplanation,
        contains_any(
            &q,
            &[
                "what does this project do",
                "what is this project",
                "what problem does this project solve",
            ],
        ),
        30,
    );
    boost_if(
        &mut scores,
        RustIntent::ArchitectureSummary,
        contains_any(
            &q,
            &[
                "architecture",
                "architectural pieces",
                "main modules",
                "responsibilities",
            ],
        ),
        30,
    );
    boost_if(
        &mut scores,
        RustIntent::CodePathExplanation,
        contains_any(
            &q,
            &[
                "trace",
                "flow",
                "path",
                "how does this work",
                "walk me through",
            ],
        ),
        30,
    );
    boost_if(
        &mut scores,
        RustIntent::EvaluativeReview,
        contains_any(
            &q,
            &[
                "review this",
                "what is wrong",
                "what's wrong",
                "risks",
                "bugs",
                "weak spots",
                "brittle",
                "health of the workspace",
            ],
        ),
        30,
    );
    boost_if(
        &mut scores,
        RustIntent::CrateSelection,
        contains_any(
            &q,
            &[
                "which crate",
                "what crate",
                "recommend a crate",
                "recommend crates",
                "crate for",
                "dependency for",
                "library for",
                "crates.io",
            ],
        ),
        30,
    );
    boost_if(
        &mut scores,
        RustIntent::ApiLookup,
        contains_any(
            &q,
            &[
                "docs.rs",
                "std::",
                "::",
                "trait ",
                "method ",
                "function ",
                "module ",
                "api ",
            ],
        ),
        25,
    );
    boost_if(
        &mut scores,
        RustIntent::ExampleLookup,
        contains_any(
            &q,
            &[
                "example",
                "examples",
                "sample code",
                "code sample",
                "snippet",
                "tutorial",
                "show me",
            ],
        ),
        20,
    );
    boost_if(
        &mut scores,
        RustIntent::CompileFix,
        contains_any(
            &q,
            &[
                "compile error",
                "compiler error",
                "cargo check",
                "does not compile",
                "build fails",
            ],
        ),
        35,
    );
    boost_if(
        &mut scores,
        RustIntent::ClippyFix,
        contains_any(&q, &["clippy", "lint", "warning cleanup"]),
        35,
    );
    boost_if(
        &mut scores,
        RustIntent::TestFix,
        contains_any(
            &q,
            &[
                "failing test",
                "tests fail",
                "cargo test",
                "repair the failing tests",
            ],
        ),
        35,
    );
    boost_if(
        &mut scores,
        RustIntent::TestCreation,
        contains_any(
            &q,
            &[
                "add tests",
                "unit test",
                "regression test",
                "improve coverage",
                "write tests",
            ],
        ),
        35,
    );
    boost_if(
        &mut scores,
        RustIntent::Refactor,
        contains_any(
            &q,
            &["refactor", "clean up", "remove duplication", "restructure"],
        ),
        30,
    );
    boost_if(
        &mut scores,
        RustIntent::FeatureImplementation,
        contains_any(
            &q,
            &[
                "implement",
                "add support",
                "add a helper",
                "create",
                "add a new",
                "update this api",
            ],
        ),
        25,
    );

    if contains_any(&q, &["fix it", "fix this", "help", "can you help"]) {
        match mode {
            SessionMode::Review => boost(&mut scores, RustIntent::EvaluativeReview, 10),
            SessionMode::Edit => boost(&mut scores, RustIntent::FeatureImplementation, 10),
            SessionMode::Ask => boost(&mut scores, RustIntent::GeneralRustQuestion, 5),
        }
    }

    if last_query.contains("clippy") {
        boost(&mut scores, RustIntent::ClippyFix, 8);
    }
    if last_query.contains("test") {
        boost(&mut scores, RustIntent::TestFix, 8);
    }
    if last_query.contains("compile") || last_query.contains("borrow checker") {
        boost(&mut scores, RustIntent::CompileFix, 8);
    }

    scores
}

pub fn default_intent_for_mode(mode: SessionMode) -> RustIntent {
    match mode {
        SessionMode::Ask => RustIntent::GeneralRustQuestion,
        SessionMode::Review => RustIntent::ProjectExplanation,
        SessionMode::Edit => RustIntent::FeatureImplementation,
    }
}

fn boost_if(scores: &mut [IntentScore], intent: RustIntent, condition: bool, amount: i32) {
    if condition {
        boost(scores, intent, amount);
    }
}

fn boost(scores: &mut [IntentScore], intent: RustIntent, amount: i32) {
    if let Some(score) = scores.iter_mut().find(|score| score.intent == intent) {
        score.score += amount;
    }
}

fn contains_any(q: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| q.contains(needle))
}

fn crate_selection_needs_clarification(query: &str) -> bool {
    let q = query.to_ascii_lowercase();

    !contains_any(
        &q,
        &[
            "tokio",
            "async",
            "await",
            "reqwest",
            "http",
            "hyper",
            "tower",
            "service",
            "middleware",
            "backoff",
            "circuit breaker",
            "database",
            "sql",
            "postgres",
            "redis",
            "grpc",
            "tonic",
        ],
    )
}

pub fn edit_request_needs_clarification(query: &str) -> bool {
    let q = query.to_ascii_lowercase();

    let looks_like_fix_request = contains_any(
        &q,
        &[
            "fix",
            "repair",
            "make this",
            "make that",
            "clean up",
            "handle",
        ],
    );

    let has_concrete_failure_signal = contains_any(
        &q,
        &[
            "compile",
            "compiler",
            "cargo check",
            "clippy",
            "warning",
            "lint",
            "test",
            "panic",
            "bug",
            "broken",
            "error",
            "failing",
            "fails",
            "wrong behavior",
        ],
    );

    let has_concrete_target_signal = contains_any(
        &q,
        &[
            ".rs",
            "config",
            "openrouter",
            "ollama",
            "retry",
            "session",
            "memory",
            "runtime",
            "request flow",
            "path",
            "paths",
            "validation",
            "env var",
            "environment variable",
            "api key",
            "base url",
            "hard coded",
        ],
    );

    looks_like_fix_request && !has_concrete_failure_signal && !has_concrete_target_signal
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_history() -> Vec<ConversationTurn> {
        Vec::new()
    }

    #[test]
    fn infers_crate_selection_for_broad_retry_question() {
        let intent = infer_initial_intent(
            "what crate should I use for retries?",
            SessionMode::Ask,
            &no_history(),
        );
        assert_eq!(intent, RustIntent::CrateSelection);
    }

    #[test]
    fn broad_retry_crate_question_needs_clarification() {
        let prompt = clarification_prompt(
            "what crate should I use for retries?",
            SessionMode::Ask,
            RustIntent::CrateSelection,
        );
        assert_eq!(
            prompt,
            Some(
                "Is this for Tokio async code, generic retry/backoff around any operation, or HTTP request retries?"
            )
        );
    }

    #[test]
    fn specific_http_retry_question_skips_clarification() {
        let prompt = clarification_prompt(
            "what crate should I use for http retries with reqwest?",
            SessionMode::Ask,
            RustIntent::CrateSelection,
        );
        assert_eq!(prompt, None);
    }

    #[test]
    fn concrete_edit_fix_request_skips_clarification() {
        let prompt = clarification_prompt(
            "fix the clippy warning in config validation",
            SessionMode::Edit,
            RustIntent::ClippyFix,
        );
        assert_eq!(prompt, None);
    }

    #[test]
    fn ambiguous_edit_fix_request_is_detected() {
        assert!(edit_request_needs_clarification("fix this"));
    }

    #[test]
    fn concrete_edit_target_skips_clarification() {
        assert!(!edit_request_needs_clarification(
            "lets fix the hard coded paths for openrouter"
        ));
        assert!(!edit_request_needs_clarification(
            "fix the config validation path"
        ));
    }

    #[test]
    fn review_request_infers_evaluative_review() {
        let intent = infer_initial_intent(
            "what is wrong with this project?",
            SessionMode::Review,
            &no_history(),
        );
        assert_eq!(intent, RustIntent::EvaluativeReview);
    }
}
