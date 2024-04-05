use std::{fs, io};
use std::collections::HashMap;
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
    let mut config_definitions: Vec<(String, Vec<String>)> = config_lines.iter().map(|config_line| {
        let definition:Vec<&str> = config_line.split(":").collect();
        let config_action = definition[0].trim().to_string();
        let key_combo = definition[1].split("+").map(|val| val.trim().to_string()).collect();
        return (config_action, key_combo);
    }).collect();

    let config_variables: HashMap<String, String> = config_definitions.iter()
        .filter(|definition| is_variable(&definition.0, &config_content))
        .map(|definition|
            (definition.0.to_owned(),
             definition.1.iter().map(|key| key.to_string()).collect::<Vec<_>>().join("+"))
        )
        .collect();

    config_definitions = config_definitions.iter().map(|definition| {
        let config_action = definition.0.to_owned();
        let mut key_combo = definition.1.to_owned();
        key_combo = key_combo.iter().map(|mut key| {
            if key.starts_with("$") {
                let var_name = String::from(key.strip_prefix("$").unwrap());
                if config_variables.contains_key(&var_name) {
                    return config_variables.get(key).unwrap().to_string();
                }
            }
            return key.to_string();
        }).collect();
        return (config_action, key_combo);
    }).collect();

    let config_action_mappings: Vec<(String, Vec<String>)> = config_definitions.iter().
        filter(|definition| !&config_variables.contains_key(&definition.0))
        .map(|definition| definition.to_owned())
        .collect();

    config_action_mappings.iter().for_each(|config_action_mapping|{
        let config_action = config_action_mapping.0.to_owned();
        let key_combo = config_action_mapping.1.to_owned();
        let action = WindowManagerAction::from_str(&config_action.as_str());
        if action.is_err() {
            let current_line = config_lines.iter().position(|line| line.starts_with(&config_action.as_str())).unwrap();
            println!("Invalid action name {} in config line {}", config_action, current_line);
            return;
        }
        let action: WindowManagerAction = WindowManagerAction::from_str(config_action.as_str()).unwrap();
        let keys = key_combo.iter().map(|key| Key::from_str(key.as_str()).unwrap()).collect();
        key_combos.push(KeyCombo::new(keys, action));
    });
    println!("Config variables: {:?}", key_combos);
}

fn is_variable(key: &str, config_content: &String) -> bool {
    let potential_var_name: String = String::from("$") + key;
    return config_content.contains(&potential_var_name.as_str());
}