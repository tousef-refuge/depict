use clap::Parser;
use crate::cli::run;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: run::Command,

    #[cfg(feature = "debug")]
    #[arg(short, long)]
    pub zipskip: bool,
}

#[cfg(feature = "debug")]
pub fn zipskip(cli: &Cli) -> bool {
    cli.zipskip
}

#[cfg(not(feature = "debug"))]
pub fn zipskip(cli: &Cli) -> bool {
    false
}