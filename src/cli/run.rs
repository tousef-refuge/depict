use clap::Subcommand;
use std::env;

#[derive(Subcommand)]
pub enum Command {}

pub fn run_command(command: Command) {
    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .expect("Failed to get binary directory");
    let project_root = exe_dir.join("../..");
}