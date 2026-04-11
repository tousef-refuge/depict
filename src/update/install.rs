use std::{env, fs};
use std::fs::File;
use std::io::Write;
use zip::ZipArchive;

use crate::github::latest_release;

const TEMP_RELEASE: &str = "depictupdate";

pub fn install_update() {
    let target_zip = if cfg!(windows) {
        "windows.zip"
    } else {
        "linux.zip"
    };

    let release = latest_release();
    let asset = release
        .assets
        .iter()
        .find(|a| a.name.ends_with(target_zip))
        .unwrap();

    //the downloader
    let response = reqwest::blocking::get(&asset.browser_download_url).unwrap();
    let zip_path = env::temp_dir().join(format!("{}.zip", TEMP_RELEASE));

    let mut out = File::create(&zip_path).unwrap();
    out.write_all(&response.bytes().unwrap()).unwrap();

    //extract
    let zip_file = File::open(&zip_path).unwrap();
    let mut archive = ZipArchive::new(zip_file).unwrap();

    let extract_dir = env::temp_dir().join(TEMP_RELEASE);
    if extract_dir.exists() {
        fs::remove_dir_all(&extract_dir).unwrap();
    }
    fs::create_dir_all(&extract_dir).unwrap();

    //extract.rs lives in our hearts :(
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let out_path = extract_dir.join(file.name());

        if file.is_dir() {
            fs::create_dir_all(&out_path).unwrap();
            continue;
        }

        if let Some(p) = out_path.parent() {
            fs::create_dir_all(p).unwrap();
        }

        let mut out_file = File::create(&out_path)
            .unwrap_or_else(|_| panic!("Failed to create file {:?}", out_path));
        std::io::copy(&mut file, &mut out_file).unwrap();
    }
}
