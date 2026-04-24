use crate::paths::project_root;
use std::fs;
use std::path::PathBuf;

fn config_json() -> PathBuf {
    project_root().join("config.json")
}

pub fn init() {
    if !config_json().exists() {
        let _ = fs::write(config_json(), " ");
    }
}