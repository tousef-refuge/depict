use colored::Colorize;
use crate::cli::commands::Command;
use crate::github::is_release;
use crate::update::install::install_update;
use crate::update::updater::run_updater;
use crate::update::versions::{current_version, latest_version};

pub fn system_command(command: Command) {
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

            if !is_release() {
                println!("{}", "You cannot run this command. This is not a github release.".red());
                return
            }

            println!("{}", "DO NOT close this terminal until it notifies you that depict has updated.".red());
            println!("Installing...");
            install_update();
            println!("Running updater...");
            run_updater(latest);
        }

        Command::Config { config_args: cmd } => {
            println!("{}", "This command doesn't do anything...yet".red());
        }

        _ => unreachable!(),
    }
}