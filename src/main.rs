mod cli;
mod update;
mod paths;

use clap::Parser;
use cli::parser::*;
use cli::run::run_command;

#[allow(unreachable_code)]
fn main() {
    let cli = Cli::parse();
    run_command(cli.command);
}
