pub mod args;
pub mod commands;
pub mod run;

use clap::Parser;
use commands::Command;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
