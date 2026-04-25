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
            "test1" => "test1".to_string(),
            "test2" => "test2".to_string(),
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