use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {}

pub fn run_command(cmd: Command) {}