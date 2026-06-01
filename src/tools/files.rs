use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::config::AppConfig;
use crate::memory::FileExcerpt;

#[derive(Debug, Clone)]
pub struct FileSearchMatch {
    pub path: String,
    pub score: i32,
    pub matching_lines: Vec<usize>,
}

pub async fn search_project_files(
    query_tokens: &[String],
    config: &AppConfig,
) -> Result<Vec<FileSearchMatch>> {
    let project_root = PathBuf::from(&config.project_root);
    let tokens = query_tokens.to_vec();

    tokio::task::spawn_blocking(move || search_project_files_sync(&project_root, &tokens))
        .await
        .context("failed to join file search task")?
}

pub async fn read_file_excerpt(
    path: &str,
    start_line: usize,
    end_line: usize,
    config: &AppConfig,
) -> Result<FileExcerpt> {
    let project_root = PathBuf::from(&config.project_root);
    let relative_path = path.to_string();

    tokio::task::spawn_blocking(move || {
        read_file_excerpt_sync(&project_root, &relative_path, start_line, end_line)
    })
    .await
    .context("failed to join file excerpt task")?
}

pub fn collect_project_files(project_root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_candidate_files(&project_root.join("src"), &mut files);

    let cargo_toml = project_root.join("Cargo.toml");
    if cargo_toml.exists() {
        files.push(cargo_toml);
    }

    files
}

fn search_project_files_sync(
    project_root: &Path,
    tokens: &[String],
) -> Result<Vec<FileSearchMatch>> {
    if tokens.is_empty() {
        return Ok(Vec::new());
    }

    let mut matches = collect_project_files(project_root)
        .into_iter()
        .filter_map(|path| score_file(&path, project_root, tokens).ok().flatten())
        .collect::<Vec<_>>();

    matches.sort_by(|a, b| b.score.cmp(&a.score));
    Ok(matches)
}

pub(crate) fn search_project_file(
    path: &Path,
    project_root: &Path,
    tokens: &[String],
) -> Result<Option<FileSearchMatch>> {
    score_file(path, project_root, tokens)
}

fn read_file_excerpt_sync(
    project_root: &Path,
    relative_path: &str,
    start_line: usize,
    end_line: usize,
) -> Result<FileExcerpt> {
    if start_line == 0 || end_line < start_line {
        bail!("invalid excerpt range {start_line}-{end_line}");
    }

    let full_path = project_root.join(relative_path);
    let content = fs::read_to_string(&full_path)
        .with_context(|| format!("failed to read file {}", full_path.display()))?;
    let lines = content.lines().collect::<Vec<_>>();
    if lines.is_empty() {
        return Ok(FileExcerpt {
            path: relative_path.to_string(),
            start_line: 1,
            end_line: 1,
            text: String::new(),
        });
    }

    let start_index = start_line.saturating_sub(1).min(lines.len() - 1);
    let end_index = end_line.min(lines.len());

    Ok(FileExcerpt {
        path: relative_path.to_string(),
        start_line: start_index + 1,
        end_line: end_index,
        text: lines[start_index..end_index].join("\n"),
    })
}

fn collect_candidate_files(dir: &Path, files: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_candidate_files(&path, files);
            continue;
        }

        let keep = matches!(
            path.extension().and_then(|ext| ext.to_str()),
            Some("rs" | "md" | "toml")
        );
        if keep {
            files.push(path);
        }
    }
}

fn score_file(
    path: &Path,
    project_root: &Path,
    tokens: &[String],
) -> Result<Option<FileSearchMatch>> {
    let relative_path = path
        .strip_prefix(project_root)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read candidate file {}", path.display()))?;
    let path_lower = relative_path.to_ascii_lowercase();
    let content_lower = content.to_ascii_lowercase();

    let mut score = 0;
    let mut matching_lines = Vec::new();

    for token in tokens {
        if path_lower.contains(token) {
            score += 8;
        }
        if content_lower.contains(token) {
            score += 3;
        }
    }

    if score == 0 {
        return Ok(None);
    }

    for (index, line) in content.lines().enumerate() {
        let line_lower = line.to_ascii_lowercase();
        if tokens.iter().any(|token| line_lower.contains(token)) {
            matching_lines.push(index + 1);
        }
    }

    Ok(Some(FileSearchMatch {
        path: relative_path,
        score,
        matching_lines,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_project_root() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        std::env::temp_dir().join(format!("rustopedia-files-{unique}"))
    }

    #[test]
    fn search_finds_relevant_file() {
        let root = temp_project_root();
        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).expect("create src dir");
        fs::write(
            src_dir.join("config.rs"),
            "pub struct AppConfig;\nimpl AppConfig { pub fn from_env() -> Self { Self } }\n",
        )
        .expect("write config");

        let matches =
            search_project_files_sync(&root, &["config".to_string(), "from_env".to_string()])
                .expect("search files");

        assert!(!matches.is_empty());
        assert_eq!(matches[0].path, "src/config.rs");

        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn read_excerpt_preserves_line_numbers() {
        let root = temp_project_root();
        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).expect("create src dir");
        fs::write(
            src_dir.join("main.rs"),
            "fn main() {\n    let answer = 42;\n    println!(\"{}\", answer);\n}\n",
        )
        .expect("write main");

        let excerpt = read_file_excerpt_sync(&root, "src/main.rs", 2, 3).expect("read excerpt");
        assert_eq!(excerpt.start_line, 2);
        assert_eq!(excerpt.end_line, 3);
        assert!(excerpt.text.contains("let answer = 42;"));
        assert!(excerpt.text.contains("println!"));

        fs::remove_dir_all(root).expect("cleanup");
    }
}
