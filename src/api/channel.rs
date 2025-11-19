use serde::Deserialize;

use crate::{Error, api::ApiClient};

#[derive(Debug, Deserialize, Clone)]
pub struct Channel {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub channel_type: u8,
    pub guild_id: Option<String>,
}

impl Channel {
    pub async fn from_id(api_client: &ApiClient, channel_id: &str) -> Result<Self, Error> {
        let url = format!("{}/channels/{channel_id}", api_client.base_url);
        let response = api_client
            .http_client
            .get(&url)
            .header("Authorization", &api_client.auth_token)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error body".to_string());

            return Err(format!("API Error: Status {status}. Details: {body}").into());
        }

        Ok(response
            .json::<Self>()
            .await
            .map_err(|e| format!("JSON Decoding Error: {e}."))?)
    }
}
