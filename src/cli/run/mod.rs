mod image;
mod system;

use crate::cli::commands::Command;
use crate::cli::run::image::image_command;
use crate::cli::run::system::system_command;

pub fn run_command(command: Command) {
    match command {
        Command::Update => system_command(command),
        _ => image_command(command),
    }
}