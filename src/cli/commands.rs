use clap::Subcommand;

//must remain in alphabetical order for -h to look nice
//everything else is in chronological order cause i hate sorting
#[derive(Subcommand)]
pub enum Command {
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
}