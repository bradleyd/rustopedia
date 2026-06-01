//! Literal-grep evidence for edit-mode queries.
//!
//! Extracts high-signal literals (URLs, env vars, host:port, snake_case,
//! PascalCase, quoted strings, numeric tokens) from the user query, then
//! shells out to ripgrep to find every occurrence in the project. The
//! resulting file:line excerpts are pinned ahead of the keyword-ranked
//! excerpts in `gather_edit_context`, so the model sees the exact lines
//! containing the value it's being asked to change.

use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashSet;
use tokio::process::Command;

use crate::config::AppConfig;
use crate::memory::FileExcerpt;
use crate::tools::files::read_file_excerpt;

const PER_LITERAL_MAX: usize = 5;
const TOTAL_MAX: usize = 15;
const CONTEXT_RADIUS: usize = 5;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Literal {
    value: String,
    specificity: u8,
}

pub async fn gather_literal_evidence(query: &str, config: &AppConfig) -> Result<Vec<FileExcerpt>> {
    let literals = extract_literals(query);
    if literals.is_empty() {
        return Ok(Vec::new());
    }

    let mut excerpts = Vec::new();
    let mut seen = HashSet::new();

    for literal in &literals {
        if excerpts.len() >= TOTAL_MAX {
            break;
        }
        let hits = match ripgrep_literal(&literal.value, config).await {
            Ok(hits) => hits,
            Err(err) => {
                eprintln!(
                    "⚠️ ripgrep failed for literal '{}': {err}",
                    literal.value
                );
                continue;
            }
        };
        for (path, line) in hits {
            if excerpts.len() >= TOTAL_MAX {
                break;
            }
            if !seen.insert((path.clone(), line)) {
                continue;
            }
            let start_line = line.saturating_sub(CONTEXT_RADIUS).max(1);
            let end_line = line + CONTEXT_RADIUS;
            match read_file_excerpt(&path, start_line, end_line, config).await {
                Ok(excerpt) => excerpts.push(excerpt),
                Err(err) => {
                    eprintln!("⚠️ excerpt fetch failed for {path}:{line}: {err}");
                }
            }
        }
    }

    Ok(excerpts)
}

async fn ripgrep_literal(literal: &str, config: &AppConfig) -> Result<Vec<(String, usize)>> {
    let output = Command::new(&config.ripgrep_bin)
        .arg("--line-number")
        .arg("--no-heading")
        .arg("--max-count")
        .arg(PER_LITERAL_MAX.to_string())
        .arg("--fixed-strings")
        .arg("--")
        .arg(literal)
        .arg(".")
        .current_dir(&config.project_root)
        .output()
        .await
        .context("failed to invoke ripgrep")?;

    if !output.status.success() {
        // exit 1 = no matches (normal); exit 2 = error
        if output.status.code() == Some(1) {
            return Ok(Vec::new());
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "ripgrep exited with status {:?}: {}",
            output.status.code(),
            stderr.trim()
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(parse_ripgrep_output(&stdout))
}

fn parse_ripgrep_output(stdout: &str) -> Vec<(String, usize)> {
    let mut results = Vec::new();
    for line in stdout.lines() {
        let mut parts = line.splitn(3, ':');
        let Some(path) = parts.next() else { continue };
        let Some(line_str) = parts.next() else { continue };
        let Ok(line_num) = line_str.trim().parse::<usize>() else {
            continue;
        };
        let cleaned = path.trim_start_matches("./").to_string();
        if cleaned.is_empty() {
            continue;
        }
        results.push((cleaned, line_num));
    }
    results
}

fn extract_literals(query: &str) -> Vec<Literal> {
    let patterns: &[(u8, &str, usize)] = &[
        // (specificity, regex, capture_group)
        (10, r"https?://\S+", 0),
        (9, r"\b[A-Z][A-Z0-9]*_[A-Z0-9_]+\b", 0),
        (8, r"\b[a-zA-Z0-9.\-]+:\d+\b", 0),
        (7, r"\b\d{4,}\b", 0),
        (6, r#""([^"]+)""#, 1),
        (6, r"'([^']+)'", 1),
        (6, r"`([^`]+)`", 1),
        (4, r"\b[a-z][a-z0-9]*_[a-z0-9_]+\b", 0),
        (4, r"\b[A-Z][a-z]+[A-Z][a-zA-Z]+\b", 0),
    ];

    let mut literals = Vec::new();
    let mut seen = HashSet::new();

    for (specificity, pattern, group) in patterns {
        let Ok(re) = Regex::new(pattern) else {
            continue;
        };
        for cap in re.captures_iter(query) {
            let Some(matched) = cap.get(*group) else {
                continue;
            };
            let value = matched.as_str().trim().trim_end_matches(|c: char| {
                matches!(c, '.' | ',' | ';' | ')' | ']' | '!' | '?')
            });
            if value.is_empty() || value.len() > 200 {
                continue;
            }
            if seen.insert(value.to_string()) {
                literals.push(Literal {
                    value: value.to_string(),
                    specificity: *specificity,
                });
            }
        }
    }

    literals.sort_by(|a, b| b.specificity.cmp(&a.specificity));
    literals
}

#[cfg(test)]
mod tests {
    use super::*;

    fn values(literals: &[Literal]) -> Vec<String> {
        literals.iter().map(|l| l.value.clone()).collect()
    }

    #[test]
    fn extracts_url_and_host_port_and_numeric() {
        let literals = extract_literals(
            "change the default ollama base url to http://127.0.0.1:11434 in src/config.rs",
        );
        let extracted = values(&literals);
        assert!(extracted.contains(&"http://127.0.0.1:11434".to_string()));
        assert!(extracted.contains(&"127.0.0.1:11434".to_string()));
        assert!(extracted.contains(&"11434".to_string()));
    }

    #[test]
    fn extracts_env_var_and_snake_case() {
        let literals = extract_literals(
            "update RUSTOPEDIA_OLLAMA_BASE_URL handling in from_env so ollama_base_url survives",
        );
        let extracted = values(&literals);
        assert!(extracted.contains(&"RUSTOPEDIA_OLLAMA_BASE_URL".to_string()));
        assert!(extracted.contains(&"from_env".to_string()));
        assert!(extracted.contains(&"ollama_base_url".to_string()));
    }

    #[test]
    fn extracts_pascal_case_and_quoted_string() {
        let literals = extract_literals(
            r#"replace "deepseek-coder-v2:latest" with the default model on AppConfig"#,
        );
        let extracted = values(&literals);
        assert!(extracted.contains(&"AppConfig".to_string()));
        assert!(extracted.contains(&"deepseek-coder-v2:latest".to_string()));
    }

    #[test]
    fn extracts_backtick_literals() {
        let literals = extract_literals("call `gather_edit_context` from `runtime.rs`");
        let extracted = values(&literals);
        assert!(extracted.contains(&"gather_edit_context".to_string()));
    }

    #[test]
    fn skips_short_words_and_stopwords() {
        let literals = extract_literals("change the url in this project");
        let extracted = values(&literals);
        assert!(extracted.is_empty(), "got: {:?}", extracted);
    }

    #[test]
    fn url_is_ranked_above_pascal_case() {
        let literals = extract_literals(
            "use https://docs.rs in AppConfig",
        );
        assert!(!literals.is_empty());
        assert_eq!(literals[0].value, "https://docs.rs");
    }

    #[test]
    fn parses_ripgrep_output_with_path_line_content() {
        let stdout = "src/config.rs:65:    .unwrap_or_else(|_| \"http://localhost:11434\".to_string()),\nsrc/llm.rs:42:        let url = format!(\"{}\", base);\n";
        let parsed = parse_ripgrep_output(stdout);
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].0, "src/config.rs");
        assert_eq!(parsed[0].1, 65);
        assert_eq!(parsed[1].0, "src/llm.rs");
        assert_eq!(parsed[1].1, 42);
    }

    #[test]
    fn parses_ripgrep_output_strips_dot_slash_prefix() {
        let stdout = "./src/foo.rs:7:some content\n";
        let parsed = parse_ripgrep_output(stdout);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].0, "src/foo.rs");
    }

    #[test]
    fn url_trailing_punctuation_stripped() {
        let literals = extract_literals("see http://example.com, it works.");
        let extracted = values(&literals);
        assert!(extracted.contains(&"http://example.com".to_string()));
    }
}
