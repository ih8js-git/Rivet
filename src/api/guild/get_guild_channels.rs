use reqwest::Client;

use crate::model::channel::Channel;

pub async fn get_guild_channels(
    client: &Client,
    token: &str,
    guild_id: &str,
) -> Result<Vec<Channel>, String> {
    let url = format!("https://discord.com/api/v10/guilds/{guild_id}/channels");
    let response = client
        .get(&url)
        .header("Authorization", token)
        .send()
        .await
        .map_err(|e| format!("API Error: {e}"))?;

    if response.status().is_success() {
        response
            .json()
            .await
            .map_err(|e| format!("JSON Error: {e}"))?
    } else {
        Err(format!("API Error: {}", response.status()))
    }
}
