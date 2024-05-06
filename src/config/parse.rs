use std::{fs, io};
use std::collections::HashMap;
use std::path::Path;
use std::process::exit;
use std::str::FromStr;

use log::error;

use crate::data::action::WindowManagerAction;
use crate::data::key::{Key, Keybind};

pub fn parse_content(config_path: &Path) -> Vec<Keybind> {
    let config_content_result: io::Result<String> = fs::read_to_string(config_path);
    if config_content_result.is_err() {
        error!("Failed to read config file");
        exit(1);
    }

    let config_content: String = config_content_result.unwrap();
    let config_lines: Vec<String> = config_content
        .split("\r\n")
        .filter(|line| line.len() > 0)
        .map(|line| String::from(line))
        .collect();
    let mut key_combos: Vec<Keybind> = Vec::new();
    let mut config_definitions: Vec<(String, Vec<String>)> = config_lines
        .iter()
        .map(|config_line| {
            let definition: Vec<&str> = config_line.split(":").collect();
            let config_action = definition[0].trim().to_string();
            let key_combo = definition[1]
                .split("+")
                .map(|val| val.trim().to_string())
                .collect();
            return (config_action, key_combo);
        })
        .collect();

    let config_variables: HashMap<String, String> = config_definitions
        .iter()
        .filter(|definition| is_variable(&definition.0, &config_content))
        .map(|definition| {
            (
                definition.0.to_owned(),
                definition
                    .1
                    .iter()
                    .map(|key| key.to_string())
                    .collect::<Vec<_>>()
                    .join("+"),
            )
        })
        .collect();

    config_definitions = config_definitions
        .iter()
        .map(|definition| {
            let config_action = definition.0.to_owned();
            let key_combo = definition.1.to_owned();
            let mut expanded_key_combo: Vec<String> = Vec::new();
            key_combo.iter().for_each(|key| {
                if !key.starts_with("$") {
                    expanded_key_combo.push(key.to_string());
                    return;
                }

                let var_name = String::from(key.strip_prefix("$").unwrap());
                if config_variables.contains_key(&var_name) {
                    let result = config_variables.get(&var_name);
                    if result.is_none() {
                        error!("Config referenced non-existent variable {}", var_name);
                        exit(100);
                    }
                    let mut expanded_var_keys: Vec<String> = result
                        .unwrap()
                        .split("+")
                        .map(|k| k.trim().to_string())
                        .collect();
                    expanded_key_combo.append(&mut expanded_var_keys);
                }
            });
            return (config_action, expanded_key_combo);
        })
        .collect();

    let config_action_mappings: Vec<(String, Vec<String>)> = config_definitions
        .iter()
        .filter(|definition| !&config_variables.contains_key(&definition.0))
        .map(|definition| definition.to_owned())
        .collect();

    config_action_mappings
        .iter()
        .for_each(|config_action_mapping| {
            let config_action = config_action_mapping.0.to_owned();
            let key_combo = config_action_mapping.1.to_owned();
            let action = WindowManagerAction::from_str(&config_action.as_str());
            if action.is_err() {
                let current_line = config_lines
                    .iter()
                    .position(|line| line.starts_with(&config_action.as_str()))
                    .unwrap();
                error!(
                    "Invalid action name {} in config line {}",
                    config_action, current_line
                );
                return;
            }
            let action= WindowManagerAction::from_str(config_action.as_str()).unwrap();
            let mut keys: Vec<Key> = key_combo
                .iter()
                .map(|key| Key::from_str(key.as_str()).unwrap())
                .collect();
            keys.sort();
            key_combos.push(Keybind::new(keys, action));
        });
    // debug!("Parsed config: {:?}", key_combos);
    return key_combos;
}

fn is_variable(key: &str, config_content: &String) -> bool {
    let potential_var_name: String = String::from("$") + key;
    let is_variable: bool = config_content.contains(&potential_var_name.as_str());
    return is_variable;
}
