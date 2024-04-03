use std::fs;
use std::path::Path;
use std::process::exit;

pub fn parse_content(config_path: &Path) {
    let content = fs::read_to_string(config_path);
    if content.is_err() {
        println!("Failed to read config file");
        exit(1);
    }
    println!("Read config content: {:}", content.unwrap());
}