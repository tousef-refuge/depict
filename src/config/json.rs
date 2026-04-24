use std::fs;
use std::path::PathBuf;
use crate::config::settings::Config;
use crate::paths::project_root;

pub fn init_config(force: bool) {
    let path = config_json();
    if !path.exists() || force {
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
    serde_json::from_str(&data).unwrap_or_else(|_| { init_config(true); Config::default() })
}

pub fn save_config(config: Config) {
    let path = config_json();
    let data = serde_json::to_string(&config).unwrap();
    fs::write(path, data).unwrap();
}

fn config_json() -> PathBuf {
    project_root().join("config.json")
}