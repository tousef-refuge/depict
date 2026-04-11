use std::process::{Command, exit, id};

use crate::paths::release_extract;

pub fn run_updater() {
    let pid = id();
    Command::new("depict-updater.exe")
        .arg(pid.to_string())
        .arg(release_extract().to_string_lossy().to_string())
        .spawn()
        .unwrap();
    exit(0);
}