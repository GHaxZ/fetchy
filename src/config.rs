// TODO: Read config file, improve error handling

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, Write};
use crossterm::style::Color;
use serde::{Deserialize, Serialize};
use crate::model::RGB;

#[derive(Serialize, Deserialize)]
struct Config {
    accent_color: Option<RGB>,
}

fn get_config_file(write: bool) -> std::io::Result<File> {
    let mut path = env::current_exe()?;
    path.pop();
    path.push("fetchy_config.json");

    OpenOptions::new()
        .read(true)
        .write(write)
        .create(write)
        .truncate(write)
        .open(path)
}

pub fn set_accent_color(color: RGB) -> std::io::Result<()> {
    let mut config_file = get_config_file(true)?;

    let mut config: Config = match serde_json::from_reader(&config_file) {
        Ok(config) => config,
        Err(_) => Config { accent_color: None },
    };

    config.accent_color = Some(color);

    config_file.seek(std::io::SeekFrom::Start(0))?;
    serde_json::to_writer(&config_file, &config)?;

    Ok(())
}

pub fn get_accent_color() -> Color {
    match get_config_file(false) {
        Ok(config_file) => {
            let config: Config = match serde_json::from_reader(config_file) {
                Ok(config) => config,
                Err(_) => Config { accent_color: None },
            };

            match config.accent_color {
                Some(color) => Color::Rgb {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                },
                None => Color::Red, // Default color
            }
        }
        Err(_) => {
            Color::Red // Default color
        }
    }
}

