use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub print_skip: bool,
    pub print_frames: bool,
    pub auto_backup: bool,
}

#[derive(Debug)]
pub enum Error { Invalid() }

impl Config {
    pub fn info(key: &str) -> Result<String, Error> {
        let val = match key {
            "print_skip" => "Print files that get skipped when processing".to_string(),
            "print_frames" => "Print processing on each individual frame of a video/gif rather than the file as a whole".to_string(),
            "auto_backup" => "When processing a file, automatically generate a backup".to_string(),
            _ => return Err(Error::Invalid()),
        };
        Ok(val)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            print_skip: true,
            print_frames: true,
            auto_backup: false,
        }
    }
}