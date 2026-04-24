use colored::Colorize;
use crate::config::json::{load_config, save_config};

pub fn edit_setting(key: String, val: String) {
    let mut config = load_config();
    match key.as_str() {
        "print_skip" => edit(&mut config.print_skip, &val, "print_skip"),
        "print_frames" => edit(&mut config.print_frames, &val, "print_frames"),
        "auto_backup" => edit(&mut config.auto_backup, &val, "auto_backup"),
        _ => println!("{} {}", "Invalid configuration key:".red(), key.red().bold()),
    }
    save_config(config);
}

fn edit<T>(key: &mut T, val: &str, key_name: &str)
where
    T: std::str::FromStr,
{
    if let Ok(parsed) = val.parse::<T>() {
        *key = parsed;
        println!(
            "{} {} {} {}",
            "Changed the setting for".blue(),
            key_name.blue(),
            "to:".blue(),
            val.blue().bold()
        );
    } else {
        println!(
            "{} {}{} {}",
            "Invalid value given for".red(),
            key_name.red(),
            ":".red(),
            val.red().bold())
    }
}