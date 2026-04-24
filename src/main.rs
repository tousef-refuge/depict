mod cli;
mod config;
mod update;

mod github;
mod paths;

use clap::Parser;
use cli::Cli;
use cli::run::run_command;
use config::json::init_config;
use update::versions::notify_update;
use update::updater::updater_cleanup;

#[allow(unreachable_code)]
fn main() {
    init_config();
    let cli = Cli::parse();
    run_command(cli.command);
    notify_update();
    updater_cleanup();
}
