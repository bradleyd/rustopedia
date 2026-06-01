mod cli;
mod config;
mod generate_prompt;
mod intents;
mod llm;
mod memory;
mod model_profile;
mod patch;
mod planner;
mod retry_loop;
mod router_llm;
mod runtime;
mod scratch;
mod session;
mod tools;

use std::process::ExitCode;

use crate::config::AppConfig;
use prettyprint::PrettyPrinter;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

#[tokio::main]
async fn main() -> ExitCode {
    let _ = dotenvy::dotenv_override();

    if std::env::args_os().len() > 1 {
        return run_one_shot().await;
    }

    run_repl().await;
    ExitCode::SUCCESS
}

async fn run_one_shot() -> ExitCode {
    let args = cli::parse();

    let mode = match cli::resolve_mode(&args.mode) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("❌ {e}");
            return ExitCode::from(3);
        }
    };

    let prompt = match cli::resolve_prompt(args.prompt.clone()) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("❌ {e}");
            return ExitCode::from(3);
        }
    };

    let mut config = AppConfig::from_env();
    if let Some(n) = args.max_retries {
        config.edit_max_retries = n;
    }
    if let Err(e) = config.validate() {
        eprintln!("❌ Invalid configuration: {e}");
        return ExitCode::from(3);
    }
    config.warn_suspicious_config();

    let runtime = match runtime::Runtime::new(config) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("❌ Failed to initialize runtime: {e:#}");
            return ExitCode::from(3);
        }
    };

    if !args.no_banner && !args.json {
        println!("{}", runtime.banner());
    }

    let summary = match runtime.handle_one_shot(&prompt, mode).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Request failed: {e:#}");
            return ExitCode::from(3);
        }
    };

    if args.json {
        let report = cli::OneShotReport {
            mode: summary.mode.as_str().to_string(),
            prompt,
            message: summary.message.clone(),
            iterations: summary.iterations,
            max_retries: summary.max_retries,
            anchor_failures: summary.anchor_failures,
            patches_proposed: summary.patches_proposed,
            patches_applicable: summary.patches_applicable,
            elapsed_ms: summary.elapsed_ms,
        };
        match serde_json::to_string(&report) {
            Ok(json) => println!("{json}"),
            Err(e) => {
                eprintln!("❌ Failed to serialize JSON report: {e}");
                return ExitCode::from(3);
            }
        }
    } else {
        let printer = PrettyPrinter::default().language("rust").build();
        match printer {
            Ok(tprint) => {
                if tprint.string(&summary.message).is_err() {
                    println!("\n{}\n", summary.message.trim());
                }
            }
            Err(_) => println!("\n{}\n", summary.message.trim()),
        }
    }

    one_shot_exit_code(&summary)
}

fn one_shot_exit_code(summary: &runtime::OneShotSummary) -> ExitCode {
    use session::SessionMode;
    if summary.mode == SessionMode::Edit {
        if summary.anchor_failures > 0 {
            return ExitCode::from(1);
        }
        if summary.patches_proposed == 0 {
            return ExitCode::from(2);
        }
    }
    ExitCode::SUCCESS
}

async fn run_repl() {
    let mut session = session::SessionState::new();
    let config = AppConfig::from_env();
    if let Err(e) = config.validate() {
        eprintln!("❌ Invalid configuration: {e}");
        return;
    }
    config.warn_suspicious_config();
    let runtime = match runtime::Runtime::new(config) {
        Ok(runtime) => runtime,
        Err(e) => {
            eprintln!("❌ Failed to initialize runtime: {e:#}");
            return;
        }
    };

    println!("{}", runtime.banner());
    let mut editor = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(e) => {
            eprintln!("❌ Failed to initialize line editor: {e}");
            return;
        }
    };

    loop {
        let input = match editor.readline("> ") {
            Ok(line) => {
                let _ = editor.add_history_entry(line.as_str());
                line
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("❌ Failed to read input: {e}");
                break;
            }
        };

        match runtime.handle_input(&input, &mut session).await {
            Ok(runtime::HandleResult::Exit) => break,
            Ok(runtime::HandleResult::Noop) => continue,
            Ok(runtime::HandleResult::Message(message)) => {
                let printer = PrettyPrinter::default().language("rust").build();
                match printer {
                    Ok(tprint) => {
                        if tprint.string(&message).is_err() {
                            println!("\n{}\n", message.trim())
                        }
                    }
                    Err(_) => println!("\n{}\n", message.trim()),
                }
            }
            Err(e) => {
                eprintln!("❌ Request failed: {e:#}");
            }
        };
    }
}
