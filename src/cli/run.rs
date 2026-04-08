use std::process::{Command as Cmd, Stdio};
use crate::cli::commands::Command;
use crate::paths::getpath::*;

pub fn run_command(command: Command, zipskip: bool) {
    match command {
        //yeah okay man what the fuck is this boilerplate are we in java
        Command::Trim { path } => {
            let mut child = Cmd::new(get_venv(zipskip))
                .arg("-u")
                .arg("-m")
                .arg("py")
                .arg("trim")
                .arg(path)
                .current_dir(&get_py(zipskip))
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("Failed to run Python module");

            child.wait().expect("Python process failed");
        },

        Command::Flip { path, axis } => {
            let mut child = Cmd::new(get_venv(zipskip))
                .arg("-u")
                .arg("-m")
                .arg("py")
                .arg("flip")
                .arg(path)
                .arg(axis.to_string())
                .current_dir(&get_py(zipskip))
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("Failed to run Python module");

            child.wait().expect("Python process failed");
        }
    }
}