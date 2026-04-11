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
