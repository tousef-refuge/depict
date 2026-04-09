use reqwest::header::{USER_AGENT, ACCEPT};
use semver::Version;
use serde::Deserialize;
use std::env;

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const GITHUB_URL: &str = "https://api.github.com/repos/tousef-refuge/depict/releases/latest";
const NULL_VERSION: Version = Version::new(0, 0, 0);

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

pub fn get_current_version() -> Version {
    Version::parse(CURRENT_VERSION).unwrap()
}

pub async fn get_latest_version() -> Version {
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

    Version::parse(latest_version).unwrap()
}
