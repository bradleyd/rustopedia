use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::config::AppConfig;

pub async fn workspace_summary(config: &AppConfig) -> Result<String> {
    let diagnostics = run_rust_analyzer(
        config,
        &[
            "-q",
            "diagnostics",
            ".",
            "--severity",
            "warning",
            "--disable-build-scripts",
            "--disable-proc-macros",
        ],
        RustAnalyzerCommandKind::Diagnostics,
    )
    .await
    .context("failed to gather rust-analyzer diagnostics")?;
    let stats = run_rust_analyzer(
        config,
        &[
            "-q",
            "analysis-stats",
            ".",
            "--disable-build-scripts",
            "--disable-proc-macros",
            "--no-test",
        ],
        RustAnalyzerCommandKind::AnalysisStats,
    )
    .await
    .context("failed to gather rust-analyzer analysis stats")?;

    let diagnostics_summary = summarize_diagnostics(&diagnostics);
    let stats_summary = summarize_stats(&stats);

    Ok(format!(
        "Rust analyzer project summary:\n{}\n{}",
        diagnostics_summary, stats_summary
    ))
}

#[derive(Clone, Copy)]
enum RustAnalyzerCommandKind {
    Diagnostics,
    AnalysisStats,
}

async fn run_rust_analyzer(
    config: &AppConfig,
    args: &[&str],
    command_kind: RustAnalyzerCommandKind,
) -> Result<String> {
    let program = config.rust_analyzer_bin.clone();
    let project_root = config.project_root.clone();
    let timeout = config.rust_analyzer_timeout();
    let args_owned = args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>();

    let output = tokio::time::timeout(
        timeout,
        tokio::task::spawn_blocking(move || {
            Command::new(&program)
                .args(&args_owned)
                .current_dir(&project_root)
                .output()
        }),
    )
    .await
    .map_err(|_| anyhow::anyhow!("rust-analyzer timed out after {}s", timeout.as_secs()))?
    .context("failed to join rust-analyzer task")?
    .context("failed to spawn rust-analyzer command")?;

    if !output.status.success() && !is_expected_diagnostics_exit(&output, command_kind) {
        bail!(
            "rust-analyzer command failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    let mut text = String::from_utf8(output.stdout)?;
    if !output.stderr.is_empty() {
        text.push('\n');
        text.push_str(&String::from_utf8_lossy(&output.stderr));
    }

    Ok(strip_control_noise(&text))
}

fn is_expected_diagnostics_exit(
    output: &std::process::Output,
    command_kind: RustAnalyzerCommandKind,
) -> bool {
    matches!(command_kind, RustAnalyzerCommandKind::Diagnostics)
        && !output.stdout.is_empty()
        && String::from_utf8_lossy(&output.stderr).contains("diagnostic error detected")
}

fn strip_control_noise(text: &str) -> String {
    let mut cleaned = String::new();
    for ch in text.chars() {
        if ch == '\u{8}' || ch == '\r' {
            continue;
        }
        cleaned.push(ch);
    }
    cleaned
}

fn summarize_diagnostics(raw: &str) -> String {
    let diagnostics = raw
        .lines()
        .filter(|line| line.contains(" at crate ") || line.starts_with("at crate "))
        .take(5)
        .map(|line| format!("- {}", line.trim()))
        .collect::<Vec<_>>();

    if diagnostics.is_empty() {
        "Diagnostics: no rust-analyzer warnings or errors reported.".to_string()
    } else {
        format!("Diagnostics:\n{}", diagnostics.join("\n"))
    }
}

fn summarize_stats(raw: &str) -> String {
    let interesting = raw
        .lines()
        .filter(|line| {
            line.contains("Workspace:")
                || line.contains("lines of code:")
                || line.contains("Total Statistics:")
                || line.starts_with("Total:")
                || line.contains("crates:")
                || line.contains("mods:")
        })
        .take(8)
        .map(|line| line.trim().to_string())
        .collect::<Vec<_>>();

    if interesting.is_empty() {
        "Analysis stats: unavailable.".to_string()
    } else {
        format!("Analysis stats:\n{}", interesting.join("\n"))
    }
}
