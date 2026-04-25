use clap::{Args, Subcommand};
use serde::Serialize;
use crate::cli::commands::Command;

#[derive(Args, Clone)]
pub struct FileArgs {
    /// Generate a backup of every image processed
    #[arg(short, long)]
    pub backup: bool,
    
    /// Compress a file after processing
    #[arg(short, long)]
    pub compress: bool,

    /// Ignore specific files
    #[arg(short, long, num_args = 1.., conflicts_with = "only", value_name = "FILES")]
    pub ignore: Option<Vec<String>>,

    /// Only process specific files
    #[arg(short, long, num_args = 1.., conflicts_with = "ignore", value_name = "FILES")]
    pub only: Option<Vec<String>>,
}

impl Command {
    pub fn file_args(&self) -> &FileArgs {
        match self {
            Command::Alpha { file_args, .. } => file_args,
            Command::Flip { file_args, .. } => file_args,
            Command::Grayscale { file_args, .. } => file_args,
            Command::Invert { file_args, .. } => file_args,
            Command::Resize { file_args, .. } => file_args,
            Command::Scale { file_args, .. } => file_args,
            Command::Trim { file_args, .. } => file_args,
            Command::Backup { file_args, .. } => file_args,
            _ => panic!("This command has no file filter"),
        }
    }
}

#[derive(Subcommand, Serialize)]
#[serde(tag = "name", rename_all = "lowercase")]
pub enum ConfigArgs {
    /// Give information about a key
    Info {
        key: String,
    },
    
    /// Change a given configuration
    Set {
        key: String,
        value: String,
    },

    /// Show the current configuration
    Show,
}