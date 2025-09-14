use reqwest::Client;

use std::{
    io::{self, Write},
    process,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::config::{self, Config};
use crate::system_info::SystemInfo;
use crate::providers::{anthropic, openai};

const MAIN_PROMPT: &str = include_str!("../prompts/main.txt");

pub struct HS {
    client: Client,
    config: Config,
    system_info: SystemInfo,
}

impl HS {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = config::read_config();

        Ok(HS {
            client: Client::new(),
            config,
            system_info: SystemInfo::new(),
        })
    }

    pub async fn translate_to_bash(
        &self,
        expression: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let prompt = MAIN_PROMPT
            .to_string()
            .replace("{{OS}}", format!("{} {}", &self.system_info.os.as_str(), &self.system_info.os_version.as_str()).as_str())
            .replace("{{SHELL}}", &self.system_info.shell)
            .replace("{{IS_ROOT}}", &self.system_info.is_root.to_string())
            .replace("{{EXPRESSION}}", expression);

        match self.config.api_type.as_str() {
            "anthropic" => {
                anthropic::call_api(&self.client, &self.config, &prompt).await
            },
            "openai" => {
                openai::call_api(&self.client, &self.config, &prompt).await
            }
            _ => Err("Unsupported API type configured".into()),
        }
    }

    pub async fn run_cli_mode(&self, expression: &str) -> Result<(), Box<dyn std::error::Error>> {
        if expression.trim().is_empty() {
            println!("Please provide an expression to translate");
            return Ok(());
        }

        match self.translate_to_bash(expression).await {
            Ok(bash_command) => {
                io::stdout().flush()?;
                println!("{}", bash_command);
            }
            Err(e) => {
                eprintln!("Error translating command: {}", e);
            }
        }

        Ok(())
    }

    pub async fn run_interactive_mode(&self) -> Result<(), Box<dyn std::error::Error>> {

        println!("HS interactive mode - Press Ctrl+H to translate");

        enable_raw_mode()?;

        let mut expression = String::new();

        loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    }) => {
                        disable_raw_mode()?;
                        println!("\nExiting...");
                        process::exit(0);
                    }
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('h'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    }) => {
                        disable_raw_mode()?;
                        print!("\nEnter expression: ");
                        io::stdout().flush()?;

                        expression.clear();
                        io::stdin().read_line(&mut expression)?;

                        if !expression.trim().is_empty() {
                            self.run_cli_mode(expression.trim()).await?;
                        }

                        println!("\nPress Ctrl+H to translate, Ctrl+C to exit");
                        enable_raw_mode()?;
                    }
                    _ => {}
                }
            }
        }
    }
}
