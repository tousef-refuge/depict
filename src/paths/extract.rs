use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[macro_export]
macro_rules! get_zip {
    ($name:literal) => {
        include_bytes!(concat!("../../", $name, ".zip"))
    }
}

pub fn zip_extract(dir: &str, zip: &[u8]) -> PathBuf {
    let extract_dir = env::temp_dir().join(format!("depict_{}", dir));
    if extract_dir.exists() {
        return extract_dir;
    }
    fs::create_dir_all(&extract_dir).unwrap();

    let zip_path = extract_dir.join(format!("{}.zip", dir));
    let mut file = File::create(&zip_path).unwrap();
    file.write_all(zip).unwrap();

    let zip_file = File::open(&zip_path).unwrap();
    let mut archive = zip::ZipArchive::new(zip_file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let out_path = extract_dir.join(file.name());

        if file.is_dir() {
            fs::create_dir_all(&out_path).unwrap();
            continue;
        }

        if let Some(p) = out_path.parent() {
            fs::create_dir_all(p).unwrap();
        }

        let mut out_file = File::create(&out_path)
            .unwrap_or_else(|_| panic!("Failed to create file {:?}", out_path));
        std::io::copy(&mut file, &mut out_file).unwrap();
    }
    extract_dir
}