mod cli;

use clap::Parser;
use cli::parser::Cli;
use cli::run::run_command;

#[allow(unreachable_code)]
fn main() {
    run_command(Cli::parse().command);
}
