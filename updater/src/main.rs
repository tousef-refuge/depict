use std::{env, fs, thread};
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use sysinfo::System;

fn main() {
    let args: Vec<String> = env::args().collect();
    let pid: usize = args[1].parse().unwrap();

    let install_dir = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let release_extract = PathBuf::from(&args[2]);

    loop {
        let mut sys = System::new_all();
        sys.refresh_all();

        if sys.process(sysinfo::Pid::from(pid)).is_none() {
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }
    thread::sleep(Duration::from_secs(5));

    for entry in fs::read_dir(release_extract).expect("Failed to read update dir") {
        let entry = entry.expect("Bad dir entry");
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name().unwrap();

            let dst = &install_dir.join(file_name);
            let backup = &install_dir.join(format!("{}.old", file_name.to_string_lossy()));

            let _ = fs::remove_file(&backup);
            let _ = fs::rename(&dst, &backup);

            fs::copy(&path, &dst).unwrap();
        }
    }

    let _ = Command::new(install_dir.join("mycli.exe"))
        .spawn();
}
