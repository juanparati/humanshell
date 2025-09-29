use std::fs;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use dirs_next::home_dir;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub endpoint: String,
    pub key: String,
    pub model: String,
    pub api_type: String,
}

pub fn prompt_for_config() -> Config {
    println!("Configuration file not found. Let's set up your API configuration.");
    
    // Prompt for API type
    print!("Enter API type (anthropic/openai) [default: anthropic]: ");
    io::stdout().flush().unwrap();

    let mut api_type = String::new();
    io::stdin().read_line(&mut api_type).unwrap();

    let api_type = api_type.trim();
    let api_type = if api_type.is_empty() {
        "anthropic".to_string()
    } else {
        api_type.to_string()
    };

    // Prompt for an API key
    print!("Enter your API key: ");
    io::stdout().flush().unwrap();
    let mut key = String::new();
    io::stdin().read_line(&mut key).unwrap();
    let key = key.trim().to_string();

    // Prompt for model (with default)
    let default_model = "claude-3-5-sonnet-20241022";
    print!("Enter model name [default: {}]: ", default_model);
    io::stdout().flush().unwrap();
    let mut model = String::new();
    io::stdin().read_line(&mut model).unwrap();
    let model = model.trim();
    let model = if model.is_empty() {
        default_model.to_string()
    } else {
        model.to_string()
    };

    // Set endpoint based on the API type
    let endpoint = match api_type.as_str() {
        "openai" => "https://api.openai.com/v1/chat/completions".to_string(),
        _ => "https://api.anthropic.com/v1/messages".to_string(),
    };

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

fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = home_dir().ok_or("Could not find home directory")?;
    let mut path = config_path;
    path.extend([".config", "humanshell"]);
    
    // Create directories if they don't exist
    fs::create_dir_all(&path)?;
    
    path.push("config.json");
    
    let config_json = serde_json::to_string_pretty(config)?;
    fs::write(path, config_json)?;

    Ok(())
}

pub fn read_config() -> Config {
    let config_path = home_dir();

    if config_path.is_none() {
        return prompt_for_config();
    }

    let mut path = config_path.unwrap();
    path.extend([".config", "humanshell", "config.json"]);

    if !path.exists() {
        return prompt_for_config();
    }

    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the config file");

    serde_json::from_str(&contents).expect("Error parsing config file")
}
