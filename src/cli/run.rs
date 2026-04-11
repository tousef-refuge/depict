use colored::Colorize;
use std::process::{Command as Cmd, Stdio};

use crate::cli::commands::{Category, Command};
use crate::paths::*;
use crate::update::versions::{current_version, latest_version};

pub async fn run_command(command: Command) {
    match command.category() {
        Category::Image => image_command(command),
        Category::System => system_command(command).await,
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

    let run_bin = exe_dir().join(if cfg!(windows) { "run.exe" } else { "run" });
    let mut child = if run_bin.exists() {
        Cmd::new(run_bin)
            .args(&args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to run .exe")
    } else {
        let root = project_root();
        let run_py = root.join("run.py");
        Cmd::new(get_venv())
            .args(["-u", run_py.to_str().unwrap()])
            .args(&args)
            .current_dir(root)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to run Python module")
    };
    child.wait().expect("Python process failed");
}

async fn system_command(command: Command) {
    match command {
        Command::Update => {
            let current = current_version();
            let latest = latest_version().await;

            if current >= latest {
                println!("Currently up to date");
                return
            }

            println!("{}", "Current version does not have built-in update installer.".red());
            println!("Download the newest release from https://github.com/tousef-refuge/depict/releases/tag/v{}", latest);
        }
        _ => {}
    }
}