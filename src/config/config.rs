use crate::constants;
use crate::logger::Logger;
use directories::ProjectDirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub default: RpcRule,
    pub classes: HashMap<String, RpcRule>,
    pub app_id: String,
}

#[derive(Deserialize, Clone)]
pub struct RpcRule {
    pub state: Option<String>,
    pub details: Option<String>,
    pub details_from_title: Option<bool>,

    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        let proj_dirs = ProjectDirs::from(
            constants::QUALIFIER,
            constants::ORGANIZATION,
            constants::APP_NAME,
        )
        .expect("Failed to get application directory");

        let data_dir = proj_dirs.data_dir();

        if !data_dir.exists() {
            fs::create_dir_all(data_dir).expect("Failed to create data dir");
        }

        let config_path = data_dir.join("config.json");

        if !config_path.exists() {
            Logger::log("Config not found, creating default config...");

            let default_config = include_str!("default-config.json");

            fs::write(&config_path, default_config).expect("Failed to write default config");
        }

        let data = fs::read_to_string(&config_path).expect("Failed to read config.json");

        Logger::log("Config file loaded!");

        serde_json::from_str(&data).expect("Invalid config.json")
    }
}
