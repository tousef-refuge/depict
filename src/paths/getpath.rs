use std::env;
use std::path::PathBuf;
use crate::get_zip;
use crate::paths::extract::zip_extract;

pub fn get_py() -> PathBuf {
    zip_extract("py", get_zip!("py"))
}

pub fn get_venv() -> PathBuf {
    let mut venv = project_root();
    if cfg!(windows) {
        //ai slop mode
        venv = venv.join("venv").join("Scripts").join("python.exe");
    } else {
        //reddit mode
        venv = venv.join("venv").join("bin").join("python");
    }
    venv
}

pub fn project_root() -> PathBuf {
    let mut dir = env::current_exe().unwrap().parent().unwrap().to_path_buf();
    loop {
        if dir.join("Cargo.toml").exists() {
            return dir;
        }

        if !dir.pop() {
            panic!("Project root not found");
        }
    }
}