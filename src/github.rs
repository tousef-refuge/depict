use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;
use std::env;

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

pub fn is_release() -> bool {
    let exe_dir = match env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
    {
        Some(dir) => dir,
        None => return false,
    };

    exe_dir.join("release.marker").exists()
}
