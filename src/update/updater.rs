use std::process::{Command, exit, id};
use semver::Version;
use crate::paths::release_extract;

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