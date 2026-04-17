use colored::Colorize;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::cli::Command;

pub fn backup_command(command: Command) {
    match command {
        Command::Restore { path: name } => {
            dir_walk(restore, Path::new(&name));
        },

        Command::Cleanup { path: name } => {
            dir_walk(cleanup, Path::new(&name));
        },

        _ => unreachable!(),
    }
}

//man i dont feel like calling python again lmao
fn dir_walk<F>(func: F, path: &Path)
where
    F: Fn(&Path),
{
    if !path.exists() {
        println!("{}", "Path does not exist".red());
        return;
    }

    for entry in WalkDir::new(&path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            func(path);
        }
    }
}

fn cleanup(path: &Path) {
    if path.extension().and_then(|e| e.to_str()) != Some("old") {
        return
    }

    fs::remove_file(path).ok();
    println!("{}{}", "Deleted: ".blue(), path.display());
}

fn restore(path: &Path) {
    if path.extension().and_then(|e| e.to_str()) != Some("old") {
        return
    }

    let file_name = match path.file_name().and_then(|n| n.to_str()) {
        Some(n) => n,
        None => return
    };

    let restored_name = file_name
        .strip_suffix(".old")
        .unwrap_or(file_name);
    let restored = path.with_file_name(restored_name);

    fs::rename(path, &restored).ok();
    println!("{}{}", "Restored: ".blue(), restored.display());
}
