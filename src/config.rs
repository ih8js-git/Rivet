use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub version: u8,
    #[serde(default)]
    pub vim_mode: bool,
    pub emoji_map: Vec<(String, String)>,
}

fn load_emojis() -> Vec<(String, String)> {
    let content = std::fs::read_to_string("emojis.json").expect("Failed to read emojis.json");
    serde_json::from_str(&content).expect("Failed to parse emojis.json")
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            vim_mode: false,
            emoji_map: load_emojis(),
        }
    }
}

pub fn load_config() -> Config {
    let app_name = "rivetui";
    match confy::load::<Config>(app_name, None) {
        Ok(mut cfg) => {
            if cfg.emoji_map.is_empty() {
                cfg.emoji_map = load_emojis();
            }
            cfg
        }
        Err(e) => {
            eprintln!("Error loading config: {e}");
            Config::default()
        }
    }
}
