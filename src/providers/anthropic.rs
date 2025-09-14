use reqwest::Client;
use serde_json::{json, Value};
use crate::config::Config;

pub async fn call_api(
    client: &Client,
    config: &Config,
    prompt: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let request_body = json!({
        "model": &config.model.as_str(),
        "max_tokens": 1000,
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ]
    });

    let response = client
        .post(&config.endpoint)
        .header("x-api-key", &config.key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Anthropic API request failed: {}: {}", response.status(), response.text().await?).into());
    }

    let response_json: Value = response.json().await?;

    if let Some(content) = response_json["content"].as_array() {
        if let Some(text) = content.get(0).and_then(|c| c["text"].as_str()) {
            return Ok(text.trim().to_string());
        }
    }

    Err("Failed to parse Anthropic API response".into())
}
