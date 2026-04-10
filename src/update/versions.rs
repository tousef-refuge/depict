use colored::Colorize;
use reqwest::header::{USER_AGENT, ACCEPT};
use semver::Version;
use serde::Deserialize;

use std::{env, fs};
use std::time::{SystemTime, UNIX_EPOCH};

const CHECK_INTERVAL: u64 = 43200; //12 hours
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const GITHUB_URL: &str = "https://api.github.com/repos/tousef-refuge/depict/releases/latest";
const NULL_VERSION: Version = Version::new(0, 0, 0);

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

pub fn current_version() -> Version {
    Version::parse(CURRENT_VERSION).unwrap()
}

pub async fn latest_version() -> Version {
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

    let response = match reqwest::Client::new()
        .get(GITHUB_URL)
        .header(USER_AGENT, "depict")
        .header(ACCEPT, "application/vnd.github.v3+json")
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return NULL_VERSION,
    };

    let release: Release = match response.json().await {
        Ok(release) => release,
        Err(_) => return NULL_VERSION,
    };
    let latest_version = release.tag_name.trim_start_matches('v');

    let _ = fs::write(cache_path, format!("{}|{}", latest_version, now));
    Version::parse(latest_version).unwrap()
}

pub async fn notify_update() {
    let latest = latest_version().await;
    let current = current_version();
    if latest > current {
        println!("A new release of depict is available ({} -> {})",
                 current.to_string().red(), latest.to_string().green());
        println!("{}{}",
                 "To update, run: ".yellow(),
                 "depict upgrade".yellow().bold());
    }
}
