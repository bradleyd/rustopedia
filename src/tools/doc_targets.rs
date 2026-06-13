//! Derive docs.rs lookup targets for edit mode.
//!
//! Edit prompts rarely name a crate API in `crate::path::Item` form, but the files the
//! edit touches almost always `use` the APIs in question. This module parses `use`
//! statements out of the relevant source, ranks them by relevance to the edit prompt,
//! and emits `crate::path::Item` strings shaped exactly as `tools::docs::build_url`
//! expects. Pure string processing over already-gathered context — no I/O, no LLM.

use std::collections::HashSet;

use regex::Regex;

/// Maximum number of docs lookups to emit per edit turn. Each is a network fetch, so we
/// keep this small and let the caller log whatever was dropped past the cap.
pub const MAX_TARGETS: usize = 2;

/// `std`/`core`/`alloc` paths resolve against doc.rust-lang.org and are always valid
/// regardless of the project's declared dependencies.
const STD_CRATES: [&str; 3] = ["std", "core", "alloc"];

/// A single importable item recovered from a `use` statement.
#[derive(Debug, Clone, PartialEq, Eq)]
struct UseCandidate {
    crate_name: String,
    /// `::`-joined path, e.g. `serde::Deserialize` — the docs_agent input shape.
    full_path: String,
    /// Final path segment, e.g. `Deserialize`.
    leaf: String,
}

/// Rank docs.rs lookup targets for an edit prompt, best first.
///
/// `sources` are full file contents (the files the edit is likely to touch); `deps` are
/// the project's declared dependency crate names (the universe of valid external crates).
/// Returns deduped `crate::path::Item` strings sorted by relevance to `query`. The list
/// is uncapped — callers should take [`MAX_TARGETS`] and log the remainder as dropped.
/// An empty result means "no confident target"; emit no tool rather than guess.
pub fn derive_targets(query: &str, sources: &[String], deps: &[String]) -> Vec<String> {
    let tokens = query_tokens(query);
    let dep_set: HashSet<&str> = deps.iter().map(String::as_str).collect();

    let mut candidates: Vec<UseCandidate> = Vec::new();
    for source in sources {
        candidates.extend(parse_use_paths(source));
    }

    rank(candidates, &tokens, &dep_set)
        .into_iter()
        .map(|c| c.full_path)
        .collect()
}

/// Parse `use` statements from a Rust source file into importable item candidates.
///
/// Line-oriented (matching the repo's existing line-based parsing style) rather than a
/// full syntax parser. Handles single paths, top-level brace groups, `self` inside
/// braces, and `as`-renames. Nested brace groups are skipped in v1.
fn parse_use_paths(source: &str) -> Vec<UseCandidate> {
    let mut out = Vec::new();

    for raw in source.lines() {
        let line = raw.trim();
        // `pub use ...` re-exports are internal API surface, not lookup targets; the
        // `"use "` prefix check naturally excludes them (they start with `pub `).
        let Some(body) = line.strip_prefix("use ") else {
            continue;
        };
        // Drop everything from the statement terminator / trailing comment onward.
        let body = body.split(';').next().unwrap_or(body).trim();
        if body.is_empty() {
            continue;
        }

        for path in expand_use_body(body) {
            if let Some(candidate) = candidate_from_path(&path) {
                out.push(candidate);
            }
        }
    }

    out
}

/// Expand a `use` body (sans `use`/`;`) into one path per imported item.
fn expand_use_body(body: &str) -> Vec<String> {
    let Some(open) = body.find('{') else {
        return vec![body.to_string()];
    };
    let prefix = body[..open].trim().trim_end_matches("::");
    let Some(close) = body.rfind('}') else {
        // Malformed; treat the pre-brace prefix as a single path.
        return vec![prefix.to_string()];
    };
    let inner = &body[open + 1..close];

    // Nested brace groups are out of scope for v1.
    if inner.contains('{') {
        return Vec::new();
    }

    inner
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(|item| {
            if item == "self" {
                prefix.to_string()
            } else {
                format!("{prefix}::{item}")
            }
        })
        .collect()
}

/// Turn a single resolved path into a candidate, applying internal/glob/rename filters.
fn candidate_from_path(path: &str) -> Option<UseCandidate> {
    // Strip an `as`-rename, keeping the real path.
    let path = path.split(" as ").next().unwrap_or(path).trim();
    let segments: Vec<&str> = path.split("::").map(str::trim).collect();

    if segments.len() < 2 {
        return None; // bare crate name carries no specific item
    }
    let crate_name = segments[0];
    let leaf = *segments.last().unwrap();

    // Internal-path roots are never docs.rs lookups.
    if matches!(crate_name, "crate" | "self" | "super") {
        return None;
    }
    // Glob imports and non-identifier segments aren't lookable items.
    if leaf == "*" || segments.iter().any(|s| !is_ident(s)) {
        return None;
    }

    Some(UseCandidate {
        crate_name: crate_name.to_string(),
        full_path: segments.join("::"),
        leaf: leaf.to_string(),
    })
}

fn is_ident(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        && s.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
}

/// Extract candidate identifiers from the edit prompt: capitalized type-like names and
/// lowercase/snake function-like names. Kept local (not reusing `literal_evidence`'s
/// private extractor) so this module stays decoupled and unit-testable — and because the
/// literal_evidence PascalCase pattern requires two humps, missing single-word types like
/// `Deserialize`.
fn query_tokens(query: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut seen = HashSet::new();
    for pattern in [r"\b[A-Z][A-Za-z0-9]+\b", r"\b[a-z][a-z0-9_]+\b"] {
        let Ok(re) = Regex::new(pattern) else {
            continue;
        };
        for m in re.find_iter(query) {
            let value = m.as_str();
            if value.len() >= 3 && seen.insert(value.to_lowercase()) {
                tokens.push(value.to_string());
            }
        }
    }
    tokens
}

/// Filter candidates to real dependencies (or std) and sort by relevance to the query.
fn rank(
    candidates: Vec<UseCandidate>,
    tokens: &[String],
    deps: &HashSet<&str>,
) -> Vec<UseCandidate> {
    let token_set: HashSet<String> = tokens.iter().map(|t| t.to_lowercase()).collect();

    let mut scored: Vec<(i32, usize, UseCandidate)> = Vec::new();
    let mut seen_paths = HashSet::new();

    for (order, candidate) in candidates.into_iter().enumerate() {
        let external_ok =
            STD_CRATES.contains(&candidate.crate_name.as_str()) || deps.contains(candidate.crate_name.as_str());
        if !external_ok {
            continue;
        }
        if !seen_paths.insert(candidate.full_path.clone()) {
            continue;
        }

        let leaf_match = token_set.contains(&candidate.leaf.to_lowercase());
        let segment_match = candidate
            .full_path
            .split("::")
            .skip(1) // already covered the leaf separately; this catches middle segments
            .any(|seg| token_set.contains(&seg.to_lowercase()));

        let score = if leaf_match {
            100
        } else if segment_match {
            50
        } else {
            continue; // no token overlap → not a confident "API I'm about to call"
        };

        scored.push((score, order, candidate));
    }

    // Higher score first; tie-break shorter path, then earlier file/line order.
    scored.sort_by(|a, b| {
        b.0.cmp(&a.0)
            .then_with(|| a.2.full_path.len().cmp(&b.2.full_path.len()))
            .then_with(|| a.1.cmp(&b.1))
    });

    scored.into_iter().map(|(_, _, c)| c).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paths(source: &str) -> Vec<String> {
        parse_use_paths(source)
            .into_iter()
            .map(|c| c.full_path)
            .collect()
    }

    #[test]
    fn parses_single_path() {
        assert_eq!(paths("use serde::Deserialize;"), vec!["serde::Deserialize"]);
    }

    #[test]
    fn expands_brace_group() {
        assert_eq!(
            paths("use tokio::sync::{Mutex, mpsc};"),
            vec!["tokio::sync::Mutex", "tokio::sync::mpsc"]
        );
    }

    #[test]
    fn keeps_std_paths() {
        assert_eq!(
            paths("use std::collections::HashMap;"),
            vec!["std::collections::HashMap"]
        );
    }

    #[test]
    fn drops_internal_roots() {
        let src = "use crate::foo::Bar;\nuse self::x::Y;\nuse super::Z;";
        assert!(paths(src).is_empty());
    }

    #[test]
    fn drops_pub_use_reexports() {
        assert!(paths("pub use foo::Bar;").is_empty());
    }

    #[test]
    fn brace_self_drops_bare_crate_keeps_item() {
        // `self` expands to the bare prefix `anyhow` (1 segment) which is dropped as a
        // bare crate name; only the specific item survives.
        assert_eq!(paths("use anyhow::{self, Context};"), vec!["anyhow::Context"]);
    }

    #[test]
    fn brace_self_keeps_deeper_prefix() {
        // Here `self` expands to `tokio::sync` (2 segments) which IS a valid item path.
        assert_eq!(
            paths("use tokio::sync::{self, Mutex};"),
            vec!["tokio::sync", "tokio::sync::Mutex"]
        );
    }

    #[test]
    fn strips_as_rename() {
        assert_eq!(paths("use foo::Bar as Baz;"), vec!["foo::Bar"]);
    }

    #[test]
    fn drops_glob_imports() {
        assert!(paths("use foo::*;").is_empty());
    }

    #[test]
    fn ignores_trailing_comment() {
        assert_eq!(
            paths("use serde::Serialize; // for the wire format"),
            vec!["serde::Serialize"]
        );
    }

    #[test]
    fn query_tokens_catches_single_word_type_and_snake() {
        let tokens = query_tokens("switch the serde Deserialize derive to a custom deserializer");
        assert!(tokens.iter().any(|t| t == "Deserialize"));
        assert!(tokens.iter().any(|t| t == "deserializer"));
    }

    #[test]
    fn derive_ranks_leaf_match_first() {
        let deps = vec!["serde".to_string(), "tokio".to_string()];
        let sources = vec!["use serde::Deserialize;\nuse tokio::sync::Mutex;".to_string()];
        let targets = derive_targets("make Config use a custom Deserialize impl", &sources, &deps);
        assert_eq!(targets.first().map(String::as_str), Some("serde::Deserialize"));
        assert!(targets.len() <= MAX_TARGETS + 5); // uncapped, but small here
    }

    #[test]
    fn derive_filters_crates_not_in_deps() {
        let deps = vec!["serde".to_string()];
        let sources = vec!["use rand::Rng;".to_string()];
        // rand is not a declared dep and not std → filtered out entirely.
        assert!(derive_targets("use Rng to shuffle", &sources, &deps).is_empty());
    }

    #[test]
    fn derive_preserves_std_even_when_not_in_deps() {
        let deps: Vec<String> = vec![];
        let sources = vec!["use std::collections::HashMap;".to_string()];
        assert_eq!(
            derive_targets("index the entries in a HashMap", &sources, &deps),
            vec!["std::collections::HashMap"]
        );
    }

    #[test]
    fn derive_empty_when_no_token_overlap() {
        let deps = vec!["serde".to_string()];
        let sources = vec!["use serde::Deserialize;".to_string()];
        // Query never mentions the API → no confident target.
        assert!(derive_targets("rename a local variable for clarity", &sources, &deps).is_empty());
    }
}
