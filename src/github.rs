use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;

pub const LATEST_RELEASE: &str = "https://api.github.com/repos/tousef-refuge/depict/releases/latest";

#[derive(Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Deserialize)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
}

pub fn latest_release() -> Release {
    reqwest::blocking::Client::new()
        .get(LATEST_RELEASE)
        .header(USER_AGENT, "depict")
        .header(ACCEPT, "application/vnd.github.v3+json")
        .send()
        .unwrap()
        .json()
        .unwrap()
}
