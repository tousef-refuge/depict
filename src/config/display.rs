use colored::Colorize;
use serde_json::Value;
use crate::config::json::load_config;
use crate::config::settings::Config;

pub fn show_settings() {
    println!("{}", "Configuration:".bold().underline());

    let value = serde_json::to_value(load_config()).unwrap();
    if let Value::Object(map) = value {
        for (key, val) in map {
            println!("  {} {}", format!("{:<15}", key).bold(), val);
        }
    }
}

pub fn show_info(key: String) {
    match Config::info(&key) {
        Ok(val) => println!("{}{} {}", key.blue(), ":".blue(), val),
        Err(_) => println!("{} {}", "Invalid configuration key:".red(), key.red().bold()),
    }
}