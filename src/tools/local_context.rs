use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::config::AppConfig;
use crate::memory::FileExcerpt;
pub async fn gather_relevant_code_excerpts(
    query: &str,
    config: &AppConfig,
) -> Result<Vec<FileExcerpt>> {
    let project_root = PathBuf::from(&config.project_root);
    let query_owned = query.to_string();

    tokio::task::spawn_blocking(move || {
        gather_relevant_code_excerpts_sync(&query_owned, &project_root)
    })
    .await
    .context("failed to join local context task")?
}

fn gather_relevant_code_excerpts_sync(
    query: &str,
    project_root: &Path,
) -> Result<Vec<FileExcerpt>> {
    let tokens = query_tokens(query);
    let file_mentions = file_mentions(query);
    if tokens.is_empty() {
        return Ok(Vec::new());
    }

    let mut matches = crate::tools::files::collect_project_files(project_root)
        .into_iter()
        .filter_map(|path| score_file(&path, project_root, &tokens, &file_mentions).transpose())
        .collect::<Result<Vec<_>>>()?;

    matches.sort_by(|a, b| b.score.cmp(&a.score));
    matches.truncate(3);

    Ok(matches
        .into_iter()
        .flat_map(|matched| matched.to_excerpts().into_iter())
        .take(4)
        .collect())
}

#[derive(Debug)]
struct FileMatch {
    score: i32,
    path: String,
    content: String,
    matching_lines: Vec<usize>,
    exact_file_match: bool,
}

impl FileMatch {
    fn to_excerpts(&self) -> Vec<FileExcerpt> {
        let lines = self.content.lines().collect::<Vec<_>>();
        if self.matching_lines.is_empty() && self.exact_file_match {
            let end_line = lines.len().clamp(1, 40);
            return vec![FileExcerpt {
                path: self.path.clone(),
                start_line: 1,
                end_line,
                text: lines[..end_line].join("\n"),
            }];
        }

        let mut excerpts = Vec::new();
        for (start_line, end_line) in excerpt_ranges(&self.matching_lines).into_iter().take(2) {
            let start_index = start_line
                .saturating_sub(1)
                .min(lines.len().saturating_sub(1));
            let end_index = end_line.min(lines.len());
            excerpts.push(FileExcerpt {
                path: self.path.clone(),
                start_line,
                end_line: end_index,
                text: lines[start_index..end_index].join("\n"),
            });
        }
        excerpts
    }
}

fn score_file(
    path: &Path,
    project_root: &Path,
    tokens: &[String],
    file_mentions: &[String],
) -> Result<Option<FileMatch>> {
    let relative_path = path
        .strip_prefix(project_root)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let exact_file_match = file_mentions
        .iter()
        .any(|mentioned| mentioned == &file_name || relative_path.ends_with(mentioned));
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read candidate file {}", path.display()))?;

    let base_match = crate::tools::files::search_project_file(path, project_root, tokens)?;

    let Some(base_match) = base_match else {
        if exact_file_match {
            return Ok(Some(FileMatch {
                score: 100,
                path: relative_path,
                content,
                matching_lines: Vec::new(),
                exact_file_match,
            }));
        }
        return Ok(None);
    };

    Ok(Some(FileMatch {
        score: base_match.score + if exact_file_match { 100 } else { 0 },
        path: base_match.path,
        content,
        matching_lines: base_match.matching_lines,
        exact_file_match,
    }))
}

fn excerpt_ranges(matching_lines: &[usize]) -> Vec<(usize, usize)> {
    let mut ranges: Vec<(usize, usize)> = Vec::new();

    for &line in matching_lines {
        let start = line.saturating_sub(2).max(1);
        let end = line + 3;

        if let Some((_, last_end)) = ranges.last_mut()
            && start <= *last_end + 1
        {
            *last_end = (*last_end).max(end);
            continue;
        }

        ranges.push((start, end));
    }

    ranges
}

fn file_mentions(query: &str) -> Vec<String> {
    query
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '.' && c != '_' && c != '/')
        .map(|part| part.trim().to_ascii_lowercase())
        .filter(|part| part.ends_with(".rs") || part.ends_with(".toml") || part.ends_with(".md"))
        .collect()
}

fn query_tokens(query: &str) -> Vec<String> {
    let stopwords = [
        "how", "does", "this", "project", "load", "the", "a", "an", "is", "for", "with", "what",
        "where", "why", "did", "you", "read", "file", "show", "me", "about",
    ];

    query
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '.' && c != '_')
        .map(|part| part.trim().to_ascii_lowercase())
        .filter(|part| part.len() >= 3 || part.ends_with(".rs"))
        .filter(|part| !stopwords.contains(&part.as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn exact_file_mentions_return_fallback_excerpt() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        let project_root = std::env::temp_dir().join(format!("rustopedia-local-context-{unique}"));
        let src_dir = project_root.join("src");
        fs::create_dir_all(&src_dir).expect("create src dir");
        fs::write(
            src_dir.join("config.rs"),
            "pub struct AppConfig;\nimpl AppConfig {\n    pub fn from_env() -> Self {\n        Self\n    }\n}\n",
        )
        .expect("write config");

        let excerpts = gather_relevant_code_excerpts_sync("did you read config.rs?", &project_root)
            .expect("gather excerpts");

        assert!(!excerpts.is_empty());
        assert_eq!(excerpts[0].path, "src/config.rs");
        assert_eq!(excerpts[0].start_line, 1);
        assert!(excerpts[0].text.contains("AppConfig"));

        fs::remove_dir_all(project_root).expect("cleanup temp project");
    }

    #[test]
    fn excerpt_ranges_dedupe_overlapping_hits() {
        let ranges = excerpt_ranges(&[10, 11, 12, 30]);
        assert_eq!(ranges, vec![(8, 15), (28, 33)]);
    }
}
