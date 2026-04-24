use crate::config::json::{load_config, save_config};

pub fn edit_setting(key: String, val: String) {
    let mut config = load_config();
    match key.as_str() {
        "test" => { config.test = val.parse().unwrap(); }
        _ => panic!("Invalid key")
    }
    save_config(config);
}