use std::process::{Command as Cmd, Stdio};
use crate::cli::commands::{Category, Command};
use crate::paths::getpath::*;

pub fn run_command(command: Command) {
    match command.category() {
        Category::Image => image_command(command),
        Category::System => system_command(command),
    }
}

fn image_command(command: Command) {
    let args: Vec<String> = match command {
        Command::Trim { path } => vec!["trim".to_string(), path],
        Command::Flip { path, axis } => vec!["flip".to_string(), path, axis.to_string()],
        Command::Scale { path, scale } => vec!["scale".to_string(), path, scale.to_string()],
        Command::Resize { path , width , height } => vec!["resize".to_string(), path, width.to_string(), height.to_string()],
        _ => Vec::new(),
    };

    let mut child = Cmd::new(get_venv())
        .args(["-u", "-m", "py"])
        .args(&args)
        .current_dir(&get_py())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to run Python module");
    child.wait().expect("Python process failed");
}

fn system_command(_command: Command) {}