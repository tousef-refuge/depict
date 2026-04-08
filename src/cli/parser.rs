use clap::Parser;
use crate::cli::commands::Command;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[cfg(feature = "debug")]
    #[arg(short, long)]
    pub zipskip: bool,
}

#[cfg(feature = "debug")]
pub fn zipskip(cli: &Cli) -> bool {
    !cli.zipskip
}

#[cfg(not(feature = "debug"))]
pub fn zipskip(_cli: &Cli) -> bool {
    false
}