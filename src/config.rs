// TODO: Read config file, improve error handling

use serde_json::{json, Value};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use crossterm::style::Color;
use crate::model::RGB;

fn get_config_file(create: bool) -> std::io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(create)
        .open("fetchy_config.json")
}

fn write_to_json(mut json: Value, content: Value) -> Value {
    if json.as_object().unwrap().contains_key("accent_color") {
        *json.as_object_mut().unwrap().get_mut("accent_color").unwrap() = content;
    } else {
        json.as_object_mut().unwrap().insert("accent_color".to_string(), content);
    }

    json
}

pub fn set_accent_color(color: RGB) {
    match get_config_file(true) {
        Ok(mut config) => {
            let mut content = String::new();
            config.read_to_string(&mut content).unwrap();

            let json: Value = serde_json::from_str(&content).unwrap() ;

            let result = write_to_json(json, json!({
                "r": color.r,
                "g": color.g,
                "b": color.b
            }));

            config.write_all(serde_json::to_string(&result).unwrap().as_bytes()).unwrap();
        }
        Err(_) => {

        }
    }

}

pub fn get_accent_color() -> Color {
    match get_config_file(false) {
        Ok(config) => {
            //Color::Rgb {}
            Color::Red
        }
        Err(_) => {
            Color::Red // Default color
        }
    }
}

