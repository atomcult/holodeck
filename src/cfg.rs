use std::collections::HashMap;

use clap::{Arg, App};
use serde_derive::Deserialize;
use anyhow::Result;

const CONFIG_REL_PATH: &str = "holodeck.toml";

pub struct Config {
    pub device: String,
    pub binds: HashMap<String, Vec<String>>,
}

#[derive(Deserialize)]
struct FileConfig {
    device: Option<String>,
    profiles: HashMap<String, HashMap<String, Vec<String>>>,
}

pub fn config() -> Result<Config> {

    let cli = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Maps buttons and keys to scripts")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Sets a custom config file")
             .takes_value(true))
        .arg(Arg::with_name("profile")
             .short("p")
             .long("profile")
             .value_name("PROFILE")
             .help("Sets the profile to use")
             .takes_value(true)
             .default_value("default"))
        .arg(Arg::with_name("device")
             .short("d")
             .long("device")
             .value_name("DEV")
             .help("Set the device to take inputs from")
             .takes_value(true))
        .get_matches();

    let config = cli.value_of("config")
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            let mut config_path = dirs::config_dir().unwrap();
            config_path.push(CONFIG_REL_PATH);
            config_path.into_os_string()
                .into_string()
                .unwrap()
        });

    let s = std::fs::read_to_string(&config)?;
    let mut file_config = toml::from_str::<FileConfig>(&s)?;

    Ok(Config {
        device: cli.value_of("device").map(|s| s.to_string())
            .or(file_config.device)
            .unwrap(),
        binds: file_config.profiles
            .remove(cli.value_of("profile").unwrap())
            .unwrap(),
    })
}
