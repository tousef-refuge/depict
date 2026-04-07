use clap::Parser;
use crate::cli::run;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: run::Command,
}