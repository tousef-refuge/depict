use std::{env, fs};
use std::path::Path;
use std::process::Command;

fn main() {
    let zip_dir = "target/zip/";
    fs::create_dir_all(&zip_dir).unwrap();

    for dir in vec!["py", "venv"] {
        let source_path = Path::new(dir);

        if !source_path.exists() {
            panic!("Required directory {} does not exist in project root. Build failed", dir);
        }
        let zip_path = format!("{}{}.zip", zip_dir, dir);

        if cfg!(windows) {
            Command::new("cmd")
                .args(["/C", "powershell", "Compress-Archive", "-Path", &source_path.to_string_lossy(), "-DestinationPath", &zip_path, "-Force"])
                .status()
                .expect(&format!("Failed to zip {}", dir));
        } else {
            Command::new("sh")
                .args(["-c", &format!("zip -r '{}' '{}'", zip_path, dir)])
                .status()
                .expect(&format!("Failed to zip {}", dir));
        }

        println!("Zipped {}", dir);
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    fs::create_dir_all(&out_dir).unwrap();
    for entry in fs::read_dir(zip_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("zip") {
            let filename = path.file_name().unwrap();
            fs::copy(&path, Path::new(&out_dir).join(filename)).unwrap();
        }
    }
}