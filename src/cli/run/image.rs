use serde_json::json;
use std::process::{Command as Cmd, Stdio};
use crate::cli::commands::Command;
use crate::paths::{exe_dir, get_venv, project_root};

pub fn image_command(command: Command) {
    let args = match command {
        Command::Trim { path } => json!({
            "name": "trim",
            "path": path,
        }),

        Command::Flip { path, axis } => json!({
            "name": "flip",
            "path": path,
            "axis": axis,
        }),

        Command::Scale { path, scale } => json!({
            "name": "scale",
            "path": path,
            "scale": scale,
        }),

        Command::Resize { path, width, height } => json!({
            "name": "resize",
            "path": path,
            "width": width,
            "height": height,
        }),

        Command::Alpha { path, alpha } => json!({
            "name": "alpha",
            "path": path,
            "alpha": alpha,
        }),

        _ => unreachable!(),
    };

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

    let mut child = cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    child.wait().expect("Python process failed");
}