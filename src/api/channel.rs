use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Channel {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub channel_type: u8,
    pub guild_id: Option<String>,
}
