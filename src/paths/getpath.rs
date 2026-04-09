use std::env;
use std::path::PathBuf;
use crate::get_zip;
use crate::paths::extract::zip_extract;

fn get_project_root() -> PathBuf {
    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .expect("Failed to get binary directory");
    exe_dir.join("../..")
}

pub fn get_py() -> PathBuf {
    zip_extract("py", get_zip!("py"))
}

pub fn get_venv() -> PathBuf {
    let mut venv = zip_extract("venv", get_zip!("venv"));
    if cfg!(windows) {
        //ai slop mode
        venv = venv.join("venv").join("Scripts").join("python.exe");
    } else {
        //reddit mode
        venv = venv.join("venv").join("bin").join("python");
    }
    venv
}