use reqwest::{Client, Response};
use serde_json::json;

use crate::Error;

pub async fn create_message(
    client: &Client,
    channel_id: &str,
    token: &str,
    content: Option<String>,
    tts: bool,
) -> Result<Response, Error> {
    let api_url = format!("https://discord.com/api/v10/channels/{channel_id}/messages?");

    let content: &str = &content.unwrap_or("".to_string());

    let payload = json!({
        "content": content,
        "tts": tts,
    });

    let response = client
        .post(&api_url)
        .header("Authorization", token)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    Ok(response)
}
