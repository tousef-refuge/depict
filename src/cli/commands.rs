use clap::Subcommand;
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
    },
    
    /// Flips an image vertical or horizontally
    Flip {
        /// Image or directory with images
        path: String,

        /// x for horizontal, y for vertical
        axis: char,
    },

    /// Resizes an image to fit the given dimensions
    Resize {
        /// Image or directory with images
        path: String,

        /// Width of the new image
        width: u32,

        /// Height of the new image
        height: u32,
    },

    /// Scales a png with respect to the given scale
    Scale {
        /// Image or directory with images
        path: String,

        /// Scale of the new image
        scale: f64,
    },

    /// Removes empty space around a transparent image
    Trim {
        /// Image or directory with images
        path: String,
    },

    /// Updates the CLI
    Update,
}
