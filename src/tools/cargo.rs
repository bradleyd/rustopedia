use std::path::Path;

use anyhow::{Context, Result, bail};
use tokio::process::Command;

use crate::config::AppConfig;

/// Outcome of running `cargo check` against a specific directory.
#[derive(Debug, Clone)]
pub struct CargoCheckOutcome {
    pub ok: bool,
    /// Concatenated stdout+stderr trimmed to the first ~120 lines. Short
    /// enough to fit in a retry directive without blowing up the prompt.
    pub output_excerpt: String,
}

/// Run `cargo check --message-format=short` against `root`. The host
/// project's `target/` is reused via `CARGO_TARGET_DIR` so subsequent
/// checks in a scratch worktree stay incremental rather than re-compiling
/// dependencies from cold every retry.
pub async fn cargo_check_at(
    root: &Path,
    shared_target_dir: Option<&Path>,
    config: &AppConfig,
) -> Result<CargoCheckOutcome> {
    let timeout = config.cargo_timeout();
    let root_owned = root.to_path_buf();
    let target_dir = shared_target_dir.map(|p| p.to_path_buf());

    let output = tokio::time::timeout(timeout, async move {
        let mut command = Command::new("cargo");
        command
            .arg("check")
            .arg("--message-format=short")
            .current_dir(&root_owned);
        if let Some(t) = target_dir {
            command.env("CARGO_TARGET_DIR", t);
        }
        command.output().await
    })
    .await
    .map_err(|_| anyhow::anyhow!("cargo check timed out after {}s", timeout.as_secs()))?
    .context("failed to spawn cargo check")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}\n{stderr}");
    let excerpt = combined
        .lines()
        .take(120)
        .collect::<Vec<_>>()
        .join("\n");

    Ok(CargoCheckOutcome {
        ok: output.status.success(),
        output_excerpt: excerpt,
    })
}

pub async fn cargo_check_summary(config: &AppConfig) -> Result<String> {
    run_cargo_summary(
        config,
        "check",
        &["--message-format=short"],
        "Cargo check: succeeded with no compiler errors.",
        "cargo check",
        CargoSummaryKind::Compiler,
    )
    .await
}

pub async fn cargo_test_summary(config: &AppConfig) -> Result<String> {
    run_cargo_summary(
        config,
        "test",
        &["--message-format=short"],
        "Cargo test: all tests passed.",
        "cargo test",
        CargoSummaryKind::Test,
    )
    .await
}

pub async fn cargo_clippy_summary(config: &AppConfig) -> Result<String> {
    run_cargo_summary(
        config,
        "clippy",
        &["--message-format=short", "--all-targets", "--no-deps"],
        "Cargo clippy: succeeded with no lint warnings.",
        "cargo clippy",
        CargoSummaryKind::Compiler,
    )
    .await
}

async fn run_cargo_summary(
    config: &AppConfig,
    subcommand: &str,
    args: &[&str],
    success_message: &str,
    command_label: &str,
    summary_kind: CargoSummaryKind,
) -> Result<String> {
    let timeout = config.cargo_timeout();
    let project_root = config.project_root.clone();
    let subcommand_name = subcommand.to_string();
    let command_label_name = command_label.to_string();
    let command_args = args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>();

    let output = tokio::time::timeout(timeout, async move {
        let mut command = Command::new("cargo");
        command.arg(&subcommand_name);
        for arg in &command_args {
            command.arg(arg);
        }
        command.current_dir(project_root).output().await
    })
    .await
    .map_err(|_| anyhow::anyhow!("{command_label} timed out after {}s", timeout.as_secs()))?
    .with_context(|| format!("failed to spawn {command_label}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}\n{stderr}");

    let diagnostics = collect_diagnostics(&combined, summary_kind);

    if output.status.success() && diagnostics.is_empty() {
        return Ok(success_message.to_string());
    }

    if diagnostics.is_empty() {
        bail!("{command_label_name} failed but no diagnostics were captured");
    }

    Ok(format!(
        "{} summary:\nStatus: {}\nDiagnostics:\n{}",
        title_case(command_label),
        if output.status.success() {
            "warnings"
        } else {
            "failed"
        },
        diagnostics.join("\n")
    ))
}

fn title_case(command_label: &str) -> String {
    let mut chars = command_label.chars();
    match chars.next() {
        Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
        None => command_label.to_string(),
    }
}

#[derive(Clone, Copy)]
enum CargoSummaryKind {
    Compiler,
    Test,
}

fn collect_diagnostics(output: &str, summary_kind: CargoSummaryKind) -> Vec<String> {
    output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| match summary_kind {
            CargoSummaryKind::Compiler => {
                line.contains("error")
                    || line.contains("warning")
                    || line.contains("-->")
                    || line.contains(':')
            }
            CargoSummaryKind::Test => {
                line.contains("error")
                    || line.contains("warning")
                    || line.contains("test result: FAILED")
                    || line.contains("failures:")
                    || line.contains("panicked at")
                    || line.starts_with("---- ")
            }
        })
        .take(12)
        .map(|line| format!("- {line}"))
        .collect()
}
