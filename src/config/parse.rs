use std::{fs, io};
use std::path::Path;
use std::process::exit;
use std::str::FromStr;
use crate::data::config::WindowManagerAction;
use crate::data::key::{Key, KeyCombo};

pub fn parse_content(config_path: &Path) {
    let config_content_result: io::Result<String> = fs::read_to_string(config_path);
    if config_content_result.is_err() {
        println!("Failed to read config file");
        exit(1);
    }

    let mut config_content: String = config_content_result.unwrap();
    let mut config_lines: Vec<String> = config_content.split("\r\n").filter(|line| line.len() > 0).map(|line|String::from(line)).collect();
    let mut key_combos: Vec<KeyCombo> = Vec::new();
    let mut config_definitions: Vec<(&str, &str)> = config_lines.iter().map(|config_line| {
        let definition:Vec<&str> = config_line.split(":").collect();
        let key = definition[0].trim();
        let value = definition[1].trim();
        return (key, value);
    }).collect();

    let config_variables: Vec<(&str, &str)> = config_definitions.iter()
        .filter(|definition| is_variable(&definition.0, &config_content))
        .map(|definition| definition.to_owned())
        .collect();

    config_variables.iter().for_each(|variable| {
        let key = variable.0.trim();
        let value = variable.1.trim();
        let var_name: String = String::from("$") + key;
        config_content = config_content.replace(var_name.as_str(), value);
    });

    let config_actions: Vec<(&str, &str)> = config_definitions.iter().filter(|definition| !&config_variables.contains(definition)).map(|definition| definition.to_owned()).collect();
    config_actions.iter().for_each(|config_action|{
        let config_key = config_action.0.trim();
        let config_value = config_action.1.trim();
        let action = WindowManagerAction::from_str(config_action.0);
        if action.is_err() {
            let current_line = config_lines.iter().position(|line| line.starts_with(config_key)).unwrap();
            println!("Invalid action name {} in config line {}", config_key, current_line);
            return;
        }
        let key: Key = Key::from_str(config_value).unwrap();
        let action: WindowManagerAction = WindowManagerAction::from_str(config_key).unwrap();
        key_combos.push(KeyCombo::new(key, action));
    });

    println!("Config variables: {:?}", key_combos);
}

fn is_variable(key: &str, config_content: &String) -> bool {
    let potential_var_name: String = String::from("$") + key;
    return config_content.contains(&potential_var_name.as_str());
}