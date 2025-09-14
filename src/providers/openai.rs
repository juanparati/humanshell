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
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "max_tokens": 1000,
        "temperature": 0.1
    });
    
    let response = client
        .post(&config.endpoint)
        .header("Authorization", format!("Bearer {}", &config.key))
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("OpenAI API request failed: {}: {}", response.status(), response.text().await?).into());
    }

    let response_json: Value = response.json().await?;

    if let Some(content) = response_json["choices"].as_array().and_then(|choices| {
        choices.get(0).and_then(|choice| choice["message"]["content"].as_str())
    }) {
        return Ok(content.trim().to_string());
    }

    Err("Failed to parse OpenAI API response".into())
}
