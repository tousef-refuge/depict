use std::process::{Command as Cmd, Stdio};
use crate::cli::commands::Command;
use crate::paths::getpath::*;

pub fn run_command(command: Command, zipskip: bool) {
    let args: Vec<String> = match command {
        Command::Trim { path } => vec!["trim".to_string(), path],
        Command::Flip { path, axis } => vec!["flip".to_string(), path, axis.to_string()],
    };

    let mut child = Cmd::new(get_venv(zipskip))
        .args(["-u", "-m", "py"])
        .args(&args)
        .current_dir(&get_py(zipskip))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to run Python module");
    child.wait().expect("Python process failed");
}