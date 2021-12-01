use std::io::prelude::*;
use std::fs::File;
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
        let mut file = File::open(&config_path)?;
        file.read_to_string(&mut s)?;
    }

   Ok(toml::from_str::<Config>(&s)?)
}
