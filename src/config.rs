use std::fs;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

static CONFIG_PATH: [&str; 2] = [".config", "humanshell"];

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub endpoint: String,
    pub key: String,
    pub model: String,
    pub api_type: String,
}

/**
 * Prompts the user for the config.
 */
pub fn prompt_for_config() -> Config {
    println!("Configuration file not found. Let's set up your API configuration.");
    
    // Prompt for API type
    let api_type = prompt_option("Enter API type (anthropic/openai)", Some("anthropic"));

    // Prompt for an API key
    let key = prompt_option("Enter API key", None);

    // Prompt for model (with default)
    let default_model = if api_type == "anthropic" { "claude-haiku-4-5" } else { "gpt-4.1-turbo" };
    let model = prompt_option("Enter model name", Some(default_model));

    // Prompt for endpoint
    // Set endpoint based on the API type
    let default_endpoint = match api_type.as_str() {
        "openai" => "https://api.openai.com/v1/chat/completions",
        _ => "https://api.anthropic.com/v1/messages",
    };

    let endpoint = prompt_option("Enter endpoint", Some(default_endpoint));

    let config = Config {
        endpoint,
        key,
        model,
        api_type,
    };

    // Save the config to file
    if let Err(e) = save_config(&config) {
        eprintln!("Warning: Could not save config file: {}", e);
    } else {
        println!("Configuration saved successfully!");
    }

    std::process::exit(0);
}

/**
 * Prompts the user for a string input.
 * If a default is provided, the user is prompted for input only if it is empty.
 */
fn prompt_option(prompt: &str, default: Option<&str>) -> String {
    if default.is_some() {
        print!("{} [default: {}]: ", prompt.to_string(), default.unwrap().to_string())
    } else {
        print!("{}: ", prompt);
    }

    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input.is_empty() && default.is_some() {
        default.unwrap().to_string()
    } else {
        input.to_string()
    }
}

/**
 * Saves the config to the home directory.
 */
fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut path = retrieve_config_path();

    // Create directories if they don't exist
    fs::create_dir_all(&path)?;
    
    path.push("config.json");
    
    let config_json = serde_json::to_string_pretty(config)?;
    fs::write(path, config_json)?;

    Ok(())
}

/**
 * Reads the config file from the home directory.
 * If the file does not exist, prompts the user for the config.
 */
pub fn read_config() -> Config {
    let mut path = retrieve_config_path();
    path.push("config.json");

    if !path.exists() {
        return prompt_for_config();
    }

    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the config file");

    serde_json::from_str(&contents).expect("Error parsing config file")
}

pub fn retrieve_config_path() -> PathBuf {
    let mut path = dirs_next::home_dir().expect("Could not find home directory");
    path.extend(CONFIG_PATH);

    // Do not allow mutation to avoid pollution.
    let path = path;

    path
}