mod cli;
mod update;
mod paths;

use clap::Parser;
use cli::parser::*;
use cli::run::run_command;
use update::versions::notify_update;

#[allow(unreachable_code)]
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    run_command(cli.command);
    notify_update().await;
}
