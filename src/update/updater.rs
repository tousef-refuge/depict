use std::fs;
use std::process::{Command, exit, id};
use semver::Version;
use crate::paths::{exe_dir, release_extract};

pub fn run_updater(version: Version) {
    let pid = id();
    Command::new("depict-updater.exe")
        .arg(pid.to_string())
        .arg(release_extract().to_string_lossy().to_string())
        .arg(version.to_string())
        .spawn()
        .unwrap();
    exit(0);
}

pub fn updater_cleanup() {
    for entry in fs::read_dir(exe_dir()).unwrap() {
        let entry = entry.expect("Bad dir entry");
        let path = entry.path();

        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.ends_with(".old") {
                let _ = fs::remove_file(&path);
            }
        }
    }
}