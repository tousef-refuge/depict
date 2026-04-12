use std::process::{Command as Cmd, Stdio};
use crate::cli::commands::Command;
use crate::paths::{exe_dir, get_venv, project_root};

pub fn image_command(command: Command) {
    let args: Vec<String> = match command {
        Command::Trim { path } => vec!["trim".to_string(), path],
        Command::Flip { path, axis } => vec!["flip".to_string(), path, axis.to_string()],
        Command::Scale { path, scale } => vec!["scale".to_string(), path, scale.to_string()],
        Command::Resize { path , width , height } => vec!["resize".to_string(), path, width.to_string(), height.to_string()],
        Command::Alpha { path, alpha } => vec!["alpha".to_string(), path, alpha.to_string()],
        _ => unreachable!(),
    };

    let run_bin = exe_dir().join(if cfg!(target_os = "windows") { "run.exe" } else { "run" });
    let mut child = if run_bin.exists() {
        Cmd::new(run_bin)
            .args(&args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to run .exe")
    } else {
        let root = project_root();
        let run_py = root.join("run.py");
        Cmd::new(get_venv())
            .args(["-u", run_py.to_str().unwrap()])
            .args(&args)
            .current_dir(root)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to run Python module")
    };
    child.wait().expect("Python process failed");
}