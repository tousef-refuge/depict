use clap::Subcommand;
use serde::Serialize;
use crate::cli::args::*;

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
