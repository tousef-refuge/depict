use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub print_skip: bool,
    pub print_frames: bool,
    pub auto_backup: bool,
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