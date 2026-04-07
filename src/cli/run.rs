use clap::Subcommand;
use std::env;
use std::path::PathBuf;
use std::process::{Command as Cmd, Stdio};

#[derive(Subcommand)]
pub enum Command {
    /// Removes empty space around a transparent image
    Trim {
        /// Image or directory with images
        path: String,
    }
}

pub fn run_command(command: Command) {
    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .expect("Failed to get binary directory");

    let project_root = exe_dir.join("../..");
    let mut venv = PathBuf::from(&project_root).join(".venv");
    if cfg!(windows) {
        //ai slop mode
        venv = venv.join("Scripts").join("python.exe");
    } else {
        //reddit mode
        venv = venv.join("bin").join("python");
    }

    match command {
        Command::Trim { path } => {
            let mut child = Cmd::new(venv)
                .arg("-u")
                .arg("-m")
                .arg("py")
                .arg("trim")
                .arg(path)
                .current_dir(&project_root)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("Failed to run Python module");

            child.wait().expect("Python process failed");
        }
    }
}