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
        file_filter: FileFilter,
    },
    
    /// Flips an image vertical or horizontally
    Flip {
        /// Image or directory with images
        path: String,

        /// x for horizontal, y for vertical
        axis: char,

        #[command(flatten)]
        #[serde(skip)]
        file_filter: FileFilter,
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
        file_filter: FileFilter,
    },

    /// Scales a png with respect to the given scale
    Scale {
        /// Image or directory with images
        path: String,

        /// Scale of the new image
        scale: f64,

        #[command(flatten)]
        #[serde(skip)]
        file_filter: FileFilter,
    },

    /// Removes empty space around a transparent image
    Trim {
        /// Image or directory with images
        path: String,

        #[command(flatten)]
        #[serde(skip)]
        file_filter: FileFilter,
    },

    /// Updates the CLI
    Update,
}

#[derive(Args, Clone)]
pub struct FileFilter {
    /// Ignore specific files
    #[arg(short, long, num_args = 1..)]
    pub ignore: Option<Vec<String>>,
}

impl Command {
    pub fn file_filter(&self) -> &FileFilter {
        match self {
            Command::Alpha { file_filter, .. } => file_filter,
            Command::Flip { file_filter, .. } => file_filter,
            Command::Resize { file_filter, .. } => file_filter,
            Command::Scale { file_filter, .. } => file_filter,
            Command::Trim { file_filter, .. } => file_filter,
            _ => panic!("This command has no file filter"),
        }
    }
}
