use std::env;
use std::path::PathBuf;
use crate::get_zip;
use crate::paths::extract::zip_extract;

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