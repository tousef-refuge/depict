use clap::CommandFactory;
use std::collections::HashMap;
use crate::cli::Cli;

pub const CMD_TYPES: &[(&str, &[&str])] = &[
    //backend
    ("color", &["alpha", "grayscale", "invert"]),
    ("files", &["backup"]),
    ("hsv", &["hue", "saturation", "value"]),
    ("orient", &["flip", "rotate"]),
    ("size", &["resize", "scale", "trim"]),

    //frontend
    ("backup", &["restore", "cleanup"]),
    ("system", &["config", "update"]),
];

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