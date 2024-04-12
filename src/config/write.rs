use std::fs;
use std::path::Path;
use std::process::exit;

use log::{error, info};

pub fn default(config_path: &Path) {
    info!(
        "Generating default config at {}",
        config_path.to_str().unwrap()
    );
    let write_config_result = fs::write(config_path, "Default config content");
    if write_config_result.is_err() {
        error!("Failed to write default config to the filesystem");
        exit(1);
    }
}
