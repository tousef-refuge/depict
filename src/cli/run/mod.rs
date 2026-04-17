mod backup;
mod image;
mod system;

use crate::cli::commands::Command;
use backup::backup_command;
use image::image_command;
use system::system_command;

pub fn run_command(command: Command) {
    match command {
        Command::Update
        | Command::Config => system_command(command),

        Command::Restore { .. }
        | Command::Cleanup { .. } => backup_command(command),

        _ => image_command(command),
    }
}