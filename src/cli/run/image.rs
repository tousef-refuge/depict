use serde_json::{json, Value};
use std::process::{Command as Cmd, Stdio};
use crate::cli::commands::{Command, FileFilter};
use crate::paths::{exe_dir, get_venv, project_root};

pub fn image_command(command: Command) {
    let args = serde_json::to_string(&command).unwrap();
    let file_filter = &command.file_filter();

    let run_bin = exe_dir().join(if cfg!(target_os = "windows") { "run.exe" } else { "run" });
    let mut cmd = if run_bin.exists() {
        Cmd::new(run_bin)
    } else {
        let root = project_root();
        let run_py = root.join("run.py");
        let mut cmd_root = Cmd::new(get_venv());
        cmd_root.args(["-u", run_py.to_str().unwrap()]).current_dir(root);
        cmd_root
    };
    cmd.arg(args.to_string());
    cmd.arg(filter_args(file_filter).to_string());

    let mut child = cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    child.wait().expect("Python process failed");
}

fn filter_args(file_filter: &FileFilter) -> Value {
    let mut data = serde_json::Map::new();

    if let Some(ignore_list) = &file_filter.ignore {
        data.insert(
            "ignore".to_string(),
            json!(ignore_list),
        );
    }

    if let Some(only_list) = &file_filter.only {
        data.insert(
            "only".to_string(),
            json!(only_list)
        );
    }

    Value::Object(data)
}