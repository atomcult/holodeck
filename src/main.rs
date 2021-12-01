use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::process::Command;

use evdev::{Device, InputEventKind};
use serde_derive::Deserialize;
use dirs;
use anyhow::Result;

#[allow(dead_code)]
mod keystate {
    pub const RELEASED: i32 = 0;
    pub const PRESSED:  i32 = 1;
    pub const HELD:     i32 = 2;
}

const CONFIG_REL_PATH: &str = "holodeck.toml";

#[derive(Deserialize)]
pub struct Config {
    pub device: Option<String>,
    pub binds: HashMap<String, Vec<String>>,
}

fn config() -> Result<Config> {
    let mut s = String::new();
    if let Some(mut config_path) = dirs::config_dir() {
        config_path.push(CONFIG_REL_PATH);
        let mut file = File::open(&config_path)?;
        file.read_to_string(&mut s)?;
    }

   Ok(toml::from_str::<Config>(&s)?)
}

fn print_supported_keys(kbd: &Device) -> Result<()> {
    if let Some(supported_keys) = kbd.supported_keys() {
        for key in supported_keys.iter() {
            println!("{:#?}", key);
        }
    }

    Ok(())
}

fn identify_keys(kbd: &mut Device) -> Result<()> {
    let event_batch = kbd.fetch_events()?;
    for event in event_batch {
        if let InputEventKind::Key(key) = event.kind() {
            if event.value() == keystate::PRESSED {
                println!("{:?}", key);
            }
        }
    }

    Ok(())
}

fn parse_hotkeys(kbd: &mut Device, binds: &HashMap<String, Vec<String>>) -> Result<()> {
    let event_batch = kbd.fetch_events()?;
    for event in event_batch {
        if let InputEventKind::Key(key) = event.kind() {
            if event.value() == keystate::PRESSED {
                if let Some(cmd) = binds.get(&format!("{:?}", key)) {
                    let mut child = Command::new(&cmd[0])
                        .args(&cmd[1..])
                        .spawn()?;

                    std::thread::spawn(move || {
                        child.wait().unwrap();
                    });
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let cfg = config()?;

    let mut kbd = Device::open(cfg.device.unwrap())?;
    kbd.grab()?;

    loop {
        parse_hotkeys(&mut kbd, &cfg.binds)?;
    }
}
