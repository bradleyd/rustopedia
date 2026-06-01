use anyhow::{Context, Result};
use tokio::process::Command;

use crate::config::AppConfig;
use crate::memory::DiffSummary;

pub async fn workspace_diff_summary(config: &AppConfig) -> Result<Option<DiffSummary>> {
    let project_root = config.project_root.clone();

    let status = Command::new("git")
        .args(["status", "--short"])
        .current_dir(&project_root)
        .output()
        .await
        .context("failed to run git status --short")?;

    if !status.status.success() {
        return Ok(None);
    }

    let status_text = String::from_utf8_lossy(&status.stdout);
    let entries = status_text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    if entries.is_empty() {
        return Ok(None);
    }

    let diff = Command::new("git")
        .args(["diff", "--stat", "--no-color"])
        .current_dir(&project_root)
        .output()
        .await
        .context("failed to run git diff --stat")?;

    let diff_text = if diff.status.success() {
        String::from_utf8_lossy(&diff.stdout).trim().to_string()
    } else {
        String::new()
    };

    let changed_files = entries
        .iter()
        .filter_map(|line| line.split_whitespace().last())
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    let mut summary_lines = vec![format!("Changed files: {}", changed_files.join(", "))];
    if !diff_text.is_empty() {
        summary_lines.push("Diff stat:".to_string());
        summary_lines.extend(diff_text.lines().take(8).map(|line| line.to_string()));
    }

    Ok(Some(DiffSummary {
        changed_files,
        summary: summary_lines.join("\n"),
    }))
}
