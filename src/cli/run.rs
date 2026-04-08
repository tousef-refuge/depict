use clap::Subcommand;
use std::process::{Command as Cmd, Stdio};
use crate::paths::getpath::*;

#[derive(Subcommand)]
pub enum Command {
    /// Removes empty space around a transparent image
    Trim {
        /// Image or directory with images
        path: String,
    }
}

pub fn run_command(command: Command) {
    match command {
        Command::Trim { path } => {
            let mut child = Cmd::new(get_venv())
                .arg("-u")
                .arg("-m")
                .arg("py")
                .arg("trim")
                .arg(path)
                .current_dir(&get_py())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("Failed to run Python module");

            child.wait().expect("Python process failed");
        }
    }
}