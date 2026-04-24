use crate::paths::project_root;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub test: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            test: false
        }
    }
}

pub fn init() {
    let path = config_json();
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        let default = Config::default();
        let data = serde_json::to_string(&default).unwrap();
        fs::write(path, data).unwrap();
    }
}

pub fn load_config() -> Config {
    let path = config_json();
    if !path.exists() {
        return Config::default();
    }

    let data = fs::read_to_string(path).unwrap();
    serde_json::from_str(&data).unwrap()
}

pub fn save_config(config: &Config) {
    let path = config_json();
    let data = serde_json::to_string(config).unwrap();
    fs::write(path, data).unwrap();
}

fn config_json() -> PathBuf {
    project_root().join("config.json")
}
