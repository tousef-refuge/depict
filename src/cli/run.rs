use colored::Colorize;
use std::process::{Command as Cmd, Stdio};

use crate::cli::commands::{Category, Command};
use crate::paths::*;
use crate::update::install::install_update;
use crate::update::updater::run_updater;
use crate::update::versions::{current_version, latest_version};

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

    let run_bin = exe_dir().join(if cfg!(target_os = "windows") { "run.exe" } else { "run" });
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

fn system_command(command: Command) {
    match command {
        Command::Update => {
            let current = current_version();
            let latest = latest_version();

            if current >= latest {
                println!("Currently up to date");
                return
            }

            if !cfg!(target_os = "windows") && !cfg!(target_os = "linux") {
                println!("{}", "This OS does not have a built-in update installer.".red());
                println!("Clone the newest CLI from https://github.com/tousef-refuge/depict/");
                return
            }

            println!("{}", "DO NOT close this terminal until it notifies you that depict has updated.".red());
            println!("Installing...");
            install_update();
            println!("Running updater...");
            run_updater(latest);
        }
        _ => {}
    }
}