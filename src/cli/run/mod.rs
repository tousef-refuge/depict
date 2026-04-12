mod image;
mod system;

use crate::cli::commands::Command;
use image::image_command;
use system::system_command;

pub fn run_command(command: Command) {
    match command {
        Command::Update => system_command(command),
        _ => image_command(command),
    }
}