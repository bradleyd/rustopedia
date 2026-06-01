//! One-shot CLI entry point. When the binary is invoked with arguments the
//! REPL is bypassed: a single query is executed and a structured result is
//! emitted to stdout. Designed for scripted evaluation of prompt/model/config
//! changes.

use std::io::{self, Read};

use anyhow::{Context, Result, anyhow, bail};
use clap::Parser;
use serde::Serialize;

use crate::session::SessionMode;

#[derive(Parser, Debug)]
#[command(name = "rustopedia", about = "Rust-first coding assistant", disable_help_subcommand = true)]
pub struct CliArgs {
    /// Mode to run the query in: ask, review, or edit.
    #[arg(long)]
    pub mode: String,

    /// Prompt text. If omitted, the prompt is read from stdin.
    #[arg(long)]
    pub prompt: Option<String>,

    /// Emit a single JSON record describing the outcome instead of pretty text.
    #[arg(long, default_value_t = false)]
    pub json: bool,

    /// Override the edit-mode retry budget for this invocation.
    #[arg(long)]
    pub max_retries: Option<u32>,

    /// Suppress the startup banner in pretty-text output.
    #[arg(long, default_value_t = false)]
    pub no_banner: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct OneShotReport {
    pub mode: String,
    pub prompt: String,
    pub message: String,
    pub iterations: u32,
    pub max_retries: u32,
    pub anchor_failures: usize,
    pub patches_proposed: usize,
    pub patches_applicable: usize,
    pub elapsed_ms: u128,
}

pub fn parse() -> CliArgs {
    CliArgs::parse()
}

pub fn resolve_mode(raw: &str) -> Result<SessionMode> {
    SessionMode::from_str(raw)
        .ok_or_else(|| anyhow!("unknown --mode value '{raw}' (expected ask|review|edit)"))
}

pub fn resolve_prompt(arg_prompt: Option<String>) -> Result<String> {
    if let Some(p) = arg_prompt {
        let trimmed = p.trim();
        if trimmed.is_empty() {
            bail!("--prompt was empty");
        }
        return Ok(trimmed.to_string());
    }

    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .context("failed to read prompt from stdin")?;
    let trimmed = buf.trim();
    if trimmed.is_empty() {
        bail!("no --prompt provided and stdin was empty");
    }
    Ok(trimmed.to_string())
}
