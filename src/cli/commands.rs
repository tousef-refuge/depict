use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    /// Removes empty space around a transparent image
    Trim {
        /// Image or directory with images
        path: String,
    }
}