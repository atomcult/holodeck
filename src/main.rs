use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

use evdev::{Device, InputEventKind};
use serde_derive::Deserialize;
use dirs;
use anyhow::Result;

mod keystate {
    pub const RELEASED: i32 = 0;
    pub const PRESSED:  i32 = 1;
    pub const HELD:     i32 = 2;
}

#[derive(Deserialize)]
pub struct Config {
    pub device: Option<String>,
    pub binds: HashMap<String, Vec<String>>,
}

fn config() -> Result<Config> {
    let mut s = String::new();
    if let Some(mut config_path) = dirs::config_dir() {
        config_path.push("deck.toml");
        let mut file = File::open(&config_path)?;
        file.read_to_string(&mut s)?;
    }

   Ok(toml::from_str::<Config>(&s)?)
}

fn main() -> Result<()> {
    let cfg: Config = config()?;

    let mut deck = Device::open(cfg.device.unwrap())?;
    deck.grab()?;

    loop {
        let event_batch = deck.fetch_events()?;
        for event in event_batch {
            if let InputEventKind::Key(key) = event.kind() {
                if event.value() == keystate::PRESSED {
                    if let Some(cmd) = cfg.binds.get(&format!("{:?}", key)) {
                        std::process::Command::new(&cmd[0])
                            .args(&cmd[1..])
                            .spawn()?;
                    }
                }
            }
        }
    }
}
