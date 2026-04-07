mod cli;

use clap::Parser;
use cli::parser::Cli;

fn main() {
    let cli = Cli::parse();
}
