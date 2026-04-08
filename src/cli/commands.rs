use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    /// Flips an image vertical or horizontally
    Flip {
        /// Image or directory with images
        path: String,

        /// x for horizontal, y for vertical
        axis: char,
    },

    /// Removes empty space around a transparent image
    Trim {
        /// Image or directory with images
        path: String,
    },
}