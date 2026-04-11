use std::io::{self, Write};

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
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
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
