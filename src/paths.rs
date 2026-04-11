use std::env;
use std::path::PathBuf;

pub fn get_venv() -> PathBuf {
    project_root()
        .join("venv")
        .join(if cfg!(target_os = "windows") { "Scripts" } else { "bin" })
        .join(if cfg!(target_os = "windows") { "python.exe" } else { "python" })
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

pub fn exe_dir() -> PathBuf {
    env::current_exe()
        .expect("Failed to get exe path")
        .parent()
        .expect("No parent directory")
        .to_path_buf()
}