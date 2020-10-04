use std::fs::File;

#[path = "../models/bot_config.rs"]
mod bot_config;
extern crate serde_yaml;
pub fn read_init_config(config_name: Option<String>) -> bot_config::BotConfig {
    let name = match config_name {
        None => "config.yaml".to_string(),
        Some(result_str) => result_str.to_string(),
    };
    let f = File::open(name).unwrap();
    return serde_yaml::from_reader(f).unwrap();
}
