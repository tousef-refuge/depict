use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = "target/zip/";
    fs::create_dir_all(&out_dir).unwrap();

    for dir in vec!["py", "venv"] {
        let source_path = Path::new(dir);

        if !source_path.exists() {
            panic!("Required directory {} does not exist in project root. Build failed", dir);
        }
        let zip_path = format!("{}{}.zip", out_dir, dir);

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
}