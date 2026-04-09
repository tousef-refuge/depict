use std::fs;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Write;
use std::path::PathBuf;

#[macro_export]
macro_rules! get_zip {
    ($name:literal) => {
        include_bytes!(concat!(env!("OUT_DIR"), "/", $name, ".zip"))
    }
}

pub fn zip_extract(dir: &str, zip: &[u8]) -> PathBuf {
    let local_data = dirs::data_local_dir()
        .expect("Failed to get local data directory")
        .join("depict");

    let zip_base = local_data.join("zips");
    fs::create_dir_all(&zip_base).unwrap();
    let zip_path = zip_base.join(format!("{}.zip", dir));
    let mut file = File::create(&zip_path).unwrap();
    file.write_all(zip).unwrap();

    //if the zip we need is already extracted then just skip extracting
    let mut hasher = DefaultHasher::new();
    zip.hash(&mut hasher);

    let hash_file = zip_base.join(format!("{}.hash", dir));
    let current_hash = hasher.finish().to_string();

    if !match fs::read_to_string(&hash_file) {
        Ok(hash) => hash != current_hash,
        Err(_) => true,
    } {
        return local_data.join(dir);
    }

    //i just realized that it nests twice for some reason
    //I will not bother fixing this.
    let extract_dir = local_data.join(dir);
    if extract_dir.exists() {
        fs::remove_dir_all(&extract_dir).unwrap();
    }
    fs::create_dir_all(&extract_dir).unwrap();

    let reader = std::io::Cursor::new(zip);
    let mut archive = zip::ZipArchive::new(reader).unwrap();

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

    fs::write(&hash_file, current_hash).unwrap();
    extract_dir
}