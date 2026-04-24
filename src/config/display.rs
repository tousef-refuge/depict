use colored::Colorize;
use serde_json::Value;
use crate::config::json::load_config;

pub fn show_settings() {
    println!("{}", "Configuration:".bold().underline());

    let value = serde_json::to_value(load_config()).unwrap();
    if let Value::Object(map) = value {
        for (key, val) in map {
            println!("  {} {}", format!("{:<11}", key).bold(), val);
        }
    }
}