use clap::{Args, Subcommand};
use serde::Serialize;

//must remain in alphabetical order for -h to look nice
//everything else is in chronological order cause i hate sorting
#[derive(Subcommand, Serialize)]
#[serde(tag = "name", rename_all = "lowercase")]
pub enum Command {
    /// Multiplies an image's opacity by the given float
    Alpha {
        /// Image or directory with images
        path: String,
        
        /// Opacity multiplier
        alpha: f64,

        #[command(flatten)]
        #[serde(skip)]
        file_args: FileArgs,
    },

    /// Creates a backup of an image
    Backup {
        /// Image or directory with images
        path: String,

        #[command(flatten)]
        #[serde(skip)]
        file_args: FileArgs,
    },

    /// Configure the CLI
    Config,

    /// Deletes a backup
    Cleanup {
        /// Backup or directory with backups
        path: String,
    },
    
    /// Flips an image vertical or horizontally
    Flip {
        /// Image or directory with images
        path: String,

        /// x for horizontal, y for vertical
        axis: char,

        #[command(flatten)]
        #[serde(skip)]
        file_args: FileArgs,
    },

    /// Converts an image to grayscale
    Grayscale {
        /// Image or directory with images
        path: String,

        #[command(flatten)]
        #[serde(skip)]
        file_args: FileArgs,
    },

    /// Invert the colors of an image
    Invert {
        /// Image or directory with images
        path: String,

        #[command(flatten)]
        #[serde(skip)]
        file_args: FileArgs,
    },

    /// Resizes an image to fit the given dimensions
    Resize {
        /// Image or directory with images
        path: String,

        /// Width of the new image
        width: u32,

        /// Height of the new image
        height: u32,

        #[command(flatten)]
        #[serde(skip)]
        file_args: FileArgs,
    },

    /// Restores a backup image back to its original form
    Restore {
        /// Backup or directory with backups
        path: String,
    },

    /// Scales a png with respect to the given scale
    Scale {
        /// Image or directory with images
        path: String,

        /// Scale of the new image
        scale: f64,

        #[command(flatten)]
        #[serde(skip)]
        file_args: FileArgs,
    },

    /// Removes empty space around a transparent image
    Trim {
        /// Image or directory with images
        path: String,

        #[command(flatten)]
        #[serde(skip)]
        file_args: FileArgs,
    },

    /// Updates the CLI
    Update,
}

#[derive(Args, Clone)]
pub struct FileArgs {
    /// Generate a backup of every image processed
    #[arg(short, long)]
    pub backup: bool,

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
