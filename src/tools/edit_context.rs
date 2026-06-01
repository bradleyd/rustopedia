use anyhow::Result;

use crate::config::AppConfig;
use crate::memory::FileExcerpt;
use crate::tools::files::FileSearchMatch;

pub struct EditContext {
    pub target_summary: String,
    pub excerpts: Vec<FileExcerpt>,
}

pub async fn gather_edit_context(query: &str, config: &AppConfig) -> Result<EditContext> {
    let literal_excerpts =
        match crate::tools::literal_evidence::gather_literal_evidence(query, config).await {
            Ok(excerpts) => excerpts,
            Err(err) => {
                eprintln!("⚠️ literal evidence failed: {err}");
                Vec::new()
            }
        };

    let tokens = query_tokens(query);
    let file_mentions = file_mentions(query);
    let mut matches = crate::tools::files::search_project_files(&tokens, config).await?;
    rerank_edit_matches(&mut matches, query, &tokens, &file_mentions);
    let top_matches = select_top_edit_matches(&matches, query, 3);

    let mut excerpts = literal_excerpts.clone();
    for matched in &top_matches {
        if let Some(first_line) = matched.matching_lines.first().copied() {
            let start_line = first_line.saturating_sub(2).max(1);
            let end_line = first_line + 3;
            let excerpt =
                crate::tools::files::read_file_excerpt(&matched.path, start_line, end_line, config)
                    .await?;
            if !excerpt_already_covered(&excerpts, &excerpt) {
                excerpts.push(excerpt);
            }
        }
    }

    let target_summary = build_target_summary(&literal_excerpts, &top_matches);

    Ok(EditContext {
        target_summary,
        excerpts,
    })
}

fn excerpt_already_covered(existing: &[FileExcerpt], candidate: &FileExcerpt) -> bool {
    existing.iter().any(|e| {
        e.path == candidate.path
            && e.start_line <= candidate.start_line
            && e.end_line >= candidate.end_line
    })
}

fn build_target_summary(
    literal_excerpts: &[FileExcerpt],
    top_matches: &[FileSearchMatch],
) -> String {
    let mut sections = Vec::new();

    if !literal_excerpts.is_empty() {
        let mut paths: Vec<String> = literal_excerpts
            .iter()
            .map(|e| format!("- {} (line {})", e.path, e.start_line))
            .collect();
        paths.dedup();
        sections.push(format!(
            "Literal-grep hits ({} excerpt(s)):\n{}",
            literal_excerpts.len(),
            paths.join("\n")
        ));
    }

    if top_matches.is_empty() {
        if sections.is_empty() {
            return "No likely edit targets were identified from the current query.".to_string();
        }
    } else {
        let lines = top_matches
            .iter()
            .map(|matched| {
                if matched.matching_lines.is_empty() {
                    format!("- {}", matched.path)
                } else {
                    format!(
                        "- {} (matching lines: {})",
                        matched.path,
                        matched
                            .matching_lines
                            .iter()
                            .take(4)
                            .map(|line| line.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        sections.push(format!("Likely edit targets (keyword-ranked):\n{lines}"));
    }

    sections.join("\n\n")
}

fn select_top_edit_matches(
    matches: &[FileSearchMatch],
    query: &str,
    max_matches: usize,
) -> Vec<FileSearchMatch> {
    if query_targets_harness(query) {
        return matches.iter().take(max_matches).cloned().collect();
    }

    let non_harness = matches
        .iter()
        .filter(|matched| !is_harness_path(&matched.path))
        .take(max_matches)
        .cloned()
        .collect::<Vec<_>>();

    if non_harness.len() >= 2 {
        return non_harness;
    }

    matches.iter().take(max_matches).cloned().collect()
}

fn query_tokens(query: &str) -> Vec<String> {
    let stopwords = [
        "fix",
        "this",
        "that",
        "the",
        "a",
        "an",
        "for",
        "with",
        "in",
        "to",
        "on",
        "make",
        "please",
        "project",
        "workspace",
        "rust",
    ];

    query
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '.' && c != '_')
        .map(|part| part.trim().to_ascii_lowercase())
        .filter(|part| part.len() >= 3 || part.ends_with(".rs"))
        .filter(|part| !stopwords.contains(&part.as_str()))
        .collect()
}

fn file_mentions(query: &str) -> Vec<String> {
    query
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '.' && c != '_' && c != '/')
        .map(|part| part.trim().to_ascii_lowercase())
        .filter(|part| part.ends_with(".rs") || part.ends_with(".toml") || part.ends_with(".md"))
        .collect()
}

fn rerank_edit_matches(
    matches: &mut [FileSearchMatch],
    query: &str,
    tokens: &[String],
    file_mentions: &[String],
) {
    let query_lower = query.to_ascii_lowercase();

    for matched in matches.iter_mut() {
        let path_lower = matched.path.to_ascii_lowercase();
        let basename = path_lower
            .rsplit('/')
            .next()
            .unwrap_or(path_lower.as_str())
            .to_string();
        let stem = basename.trim_end_matches(".rs").trim_end_matches(".toml");
        let explicit_file_reference = file_mentions
            .iter()
            .any(|mentioned| mentioned == &basename || path_lower.ends_with(mentioned));

        let mut adjusted = matched.score;
        adjusted += (matched.matching_lines.len().min(6) as i32) * 4;

        for token in tokens {
            if stem == token || basename == *token {
                adjusted += 30;
            } else if basename.contains(token) {
                adjusted += 12;
            }
        }

        if explicit_file_reference {
            adjusted += 100;
        }

        if contains_any(&query_lower, &["test", "tests", "#[test]"])
            && (path_lower.contains("/test") || basename.contains("test"))
        {
            adjusted += 40;
        }

        if is_harness_file(&basename) && !explicit_file_reference {
            adjusted -= 25;
        }
        if path_lower.starts_with("src/tools/") && !explicit_file_reference {
            adjusted -= 10;
        }

        matched.score = adjusted;
    }

    matches.sort_by(|a, b| b.score.cmp(&a.score));
}

fn query_targets_harness(query: &str) -> bool {
    contains_any(
        &query.to_ascii_lowercase(),
        &[
            "runtime",
            "session",
            "memory",
            "planner",
            "intent",
            "prompt",
            "harness",
            "edit context",
            "local context",
            "project overview",
            "runtime.rs",
            "session.rs",
            "memory.rs",
            "planner.rs",
            "intents.rs",
            "generate_prompt.rs",
            "router_llm.rs",
        ],
    )
}

fn is_harness_path(path: &str) -> bool {
    let basename = path.rsplit('/').next().unwrap_or(path).to_ascii_lowercase();
    is_harness_file(&basename)
}

fn is_harness_file(basename: &str) -> bool {
    matches!(
        basename,
        "runtime.rs"
            | "intents.rs"
            | "planner.rs"
            | "session.rs"
            | "generate_prompt.rs"
            | "router_llm.rs"
            | "memory.rs"
            | "mod.rs"
            | "edit_context.rs"
            | "files.rs"
            | "local_context.rs"
            | "project_overview.rs"
            | "rust_analyzer.rs"
            | "cargo.rs"
    )
}

fn contains_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rerank_prefers_domain_files_over_harness_files() {
        let mut matches = vec![
            FileSearchMatch {
                path: "src/runtime.rs".to_string(),
                score: 20,
                matching_lines: vec![5],
            },
            FileSearchMatch {
                path: "src/config.rs".to_string(),
                score: 18,
                matching_lines: vec![30, 105],
            },
        ];

        rerank_edit_matches(
            &mut matches,
            "lets fix the config validation path",
            &query_tokens("lets fix the config validation path"),
            &file_mentions("lets fix the config validation path"),
        );

        assert_eq!(matches[0].path, "src/config.rs");
    }

    #[test]
    fn rerank_boosts_testish_paths_for_test_queries() {
        let mut matches = vec![
            FileSearchMatch {
                path: "src/runtime.rs".to_string(),
                score: 18,
                matching_lines: vec![40],
            },
            FileSearchMatch {
                path: "src/retry_tests.rs".to_string(),
                score: 16,
                matching_lines: vec![10],
            },
        ];

        rerank_edit_matches(
            &mut matches,
            "lets fix the breaking test",
            &query_tokens("lets fix the breaking test"),
            &file_mentions("lets fix the breaking test"),
        );

        assert_eq!(matches[0].path, "src/retry_tests.rs");
    }

    #[test]
    fn select_top_edit_matches_filters_harness_noise_for_domain_query() {
        let matches = vec![
            FileSearchMatch {
                path: "src/llm.rs".to_string(),
                score: 100,
                matching_lines: vec![32],
            },
            FileSearchMatch {
                path: "src/config.rs".to_string(),
                score: 95,
                matching_lines: vec![66],
            },
            FileSearchMatch {
                path: "src/intents.rs".to_string(),
                score: 90,
                matching_lines: vec![485],
            },
        ];

        let selected =
            select_top_edit_matches(&matches, "lets fix the hard coded paths for openrouter", 3);
        let paths = selected.into_iter().map(|m| m.path).collect::<Vec<_>>();

        assert!(paths.contains(&"src/llm.rs".to_string()));
        assert!(paths.contains(&"src/config.rs".to_string()));
        assert!(!paths.contains(&"src/intents.rs".to_string()));
    }

    #[test]
    fn select_top_edit_matches_keeps_harness_targets_when_query_points_there() {
        let matches = vec![
            FileSearchMatch {
                path: "src/memory.rs".to_string(),
                score: 100,
                matching_lines: vec![20],
            },
            FileSearchMatch {
                path: "src/session.rs".to_string(),
                score: 95,
                matching_lines: vec![50],
            },
            FileSearchMatch {
                path: "src/config.rs".to_string(),
                score: 90,
                matching_lines: vec![60],
            },
        ];

        let selected =
            select_top_edit_matches(&matches, "lets refactor the session memory handling", 3);
        let paths = selected.into_iter().map(|m| m.path).collect::<Vec<_>>();

        assert!(paths.contains(&"src/memory.rs".to_string()));
        assert!(paths.contains(&"src/session.rs".to_string()));
    }
}
