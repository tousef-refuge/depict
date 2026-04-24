use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub test: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            test: false
        }
    }
}