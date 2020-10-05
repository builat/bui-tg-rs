use std::fs::File;
extern crate serde_yaml;
use crate::bot::models::BotConfig;

pub fn read_init_config(config_name: Option<String>) -> Result<BotConfig, &'static str> {
    let name = match config_name {
        None => "config.yaml".to_string(),
        Some(result_str) => result_str.to_string(),
    };

    let file = File::open(name);

    if file.is_err() {
        return Err("File not found");
    }

    let parsed_config = serde_yaml::from_reader(file.unwrap());

    if parsed_config.is_err() {
        return Err("Failed to parse config");
    }
    return Ok(parsed_config.unwrap());
}
