use std::fs::File;
extern crate serde_yaml;

use crate::bot::models::BotConfig;

pub fn read_init_config(config_name: Option<String>) -> BotConfig {
    let name = match config_name {
        None => "config.yaml".to_string(),
        Some(result_str) => result_str.to_string(),
    };
    let f = File::open(name).unwrap();
    return serde_yaml::from_reader(f).unwrap();
}
