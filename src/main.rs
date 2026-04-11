mod cli;
mod update;

mod github;
mod paths;

use clap::Parser;
use cli::parser::*;
use cli::run::run_command;
use update::versions::notify_update;
use update::updater::updater_cleanup;

#[allow(unreachable_code)]
fn main() {
    let cli = Cli::parse();
    run_command(cli.command);
    notify_update();
    updater_cleanup();
}
