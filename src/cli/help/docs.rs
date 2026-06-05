use clap::CommandFactory;
use std::collections::HashMap;
use crate::cli::Cli;

pub fn command_docs() -> HashMap<String, String> {
    Cli::command()
        .get_subcommands()
        .map(|cmd| {
            (
                cmd.get_name().to_string(),
                cmd.get_about().unwrap().to_string(),
            )
        })
        .collect()
}