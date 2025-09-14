use std::fs;
use serde::{Deserialize, Serialize};
use dirs_next::home_dir;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub endpoint: String,
    pub key: String,
    pub model: String,
    pub api_type: String,
}

pub fn read_config() -> Config {
    let default_config = Config {
        endpoint: String::from("https://api.anthropic.com/v1/messages"),
        key: String::new(),
        model: String::from("claude-3-5-sonnet-20241022"),
        api_type: String::from("anthropic"),
    };

    let config_path = home_dir();

    if config_path.is_none() {
        return default_config;
    }

    let mut path = config_path.unwrap();
    path.extend([".config", "hs", "config.json"]);

    if !path.exists() {
        return default_config;
    }

    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the config file");

    let mut config: Config = serde_json::from_str(&contents).expect("Error parsing config file");

    // Set default openai_endpoint if api_type is openai and openai_endpoint is not set
    if config.api_type == "openai" {
        config.endpoint = "https://api.openai.com/v1/chat/completions".to_string();
    }

    config
}
