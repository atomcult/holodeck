use std::collections::HashMap;
use std::process::Command;

use evdev::{Device, InputEventKind};
use anyhow::Result;

mod cfg;

#[allow(dead_code)]
mod keystate {
    pub const RELEASED: i32 = 0;
    pub const PRESSED:  i32 = 1;
    pub const HELD:     i32 = 2;
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
    let cfg = cfg::config()?;

    let mut kbd = Device::open(cfg.device.unwrap())?;
    kbd.grab()?;

    loop {
        parse_hotkeys(&mut kbd, &cfg.profiles["default"])?;
    }
}
