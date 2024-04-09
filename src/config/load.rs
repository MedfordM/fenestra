use crate::config;
use std::path::Path;

pub fn ensure_exists(config_path: &Path) {
    if !config_path.exists() {
        println!(
            "Unable to locate config file at {}",
            config_path.to_str().unwrap()
        );
        config::write::default(config_path);
    }
}
