mod config;
mod generate_prompt;
mod llm;
mod planner;
mod router_llm;
mod runtime;
mod session;
mod tools;

use crate::config::AppConfig;
use prettyprint::PrettyPrinter;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

#[tokio::main]
async fn main() {
    let mut session = session::SessionState::new();
    let config = AppConfig::from_env();
    if let Err(e) = config.validate() {
        eprintln!("❌ Invalid configuration: {e}");
        return;
    }
    let runtime = match runtime::Runtime::new(config) {
        Ok(runtime) => runtime,
        Err(e) => {
            eprintln!("❌ Failed to initialize runtime: {e}");
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
            Err(ReadlineError::Interrupted) => continue,
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
                eprintln!("❌ Request failed: {e}");
            }
        };
    }
}
