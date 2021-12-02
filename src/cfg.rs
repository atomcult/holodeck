use std::collections::HashMap;

use serde_derive::Deserialize;
use anyhow::Result;

const CONFIG_REL_PATH: &str = "holodeck.toml";

#[derive(Deserialize)]
pub struct Config {
    pub device: Option<String>,
    pub binds: HashMap<String, Vec<String>>,
}

pub fn config() -> Result<Config> {
    let mut s = String::new();
    if let Some(mut config_path) = dirs::config_dir() {
        config_path.push(CONFIG_REL_PATH);
        s = std::fs::read_to_string(&config_path)?;
    }

   Ok(toml::from_str::<Config>(&s)?)
}
