use colored::Colorize;
use semver::Version;

use crate::github::*;
use std::{env, fs};
use std::time::{SystemTime, UNIX_EPOCH};

const CHECK_INTERVAL: u64 = 43200; //12 hours
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn current_version() -> Version {
    Version::parse(CURRENT_VERSION).unwrap()
}

pub fn latest_version() -> Version {
    //dont spam github with requests
    let cache_path = env::temp_dir().join("depict_check_update");

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    if let Ok(contents) = fs::read_to_string(&cache_path) {
        let parts: Vec<&str> = contents.trim().split('|').collect();
        if parts.len() == 2 {
            let version = parts[0];
            let last_check = parts[1].parse::<u64>().unwrap();

            if now - last_check < CHECK_INTERVAL {
                return Version::parse(version).unwrap();
            }
        }
    }

    let latest_release = latest_release();
    let latest_version = latest_release.tag_name.trim_start_matches('v');

    let _ = fs::write(cache_path, format!("{}|{}", latest_version, now));
    Version::parse(latest_version).unwrap()
}

pub fn notify_update() {
    let latest = latest_version();
    let current = current_version();
    if latest > current {
        println!("A new release of depict is available ({} -> {})",
                 current.to_string().red(), latest.to_string().green());
        println!("{}{}",
                 "To update, run: ".yellow(),
                 "depict upgrade".yellow().bold());
    }
}
