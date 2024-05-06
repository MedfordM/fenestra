use crate::data::action::Execute;
use crate::data::key::{Key, KeyAction::{PRESS, RELEASE}, Keybind, KeyPress, WINDOWS_KEY_CODE};

pub struct KeyManager {
    keybinds: Vec<Keybind>,
    pressed_keys: Vec<Key>,
}

impl KeyManager {
    pub fn new(keybinds: Vec<Keybind>) -> Self {
        Self { keybinds, pressed_keys: Vec::new() }
    }
    
    pub fn get_keybinds(&self) -> &Vec<Keybind> {
        &self.keybinds
    }

    pub fn get_pressed_keys(&self) -> &Vec<Keybind> {
        &self.keybinds
    }
    
    // Returns a bool based on if the keypress should be propagated to the rest of the key listeners
    pub fn handle_keypress(&mut self, key_press: KeyPress) -> bool {
        match key_press.action {
            PRESS => {
                // User pressed a key, add it to pressed_keys
                if !self.pressed_keys.contains(&key_press.key) {
                    self.pressed_keys.push(key_press.clone().key);
                }
                // Suppress every press of the WIN key
                match key_press.key.code { 
                    WINDOWS_KEY_CODE => true,
                    _ => false
                }
            },
            RELEASE => {
                // Attempt to find a matching keybind
                for keybind in &self.keybinds {
                    let mut pressed_keys = self.pressed_keys.clone();
                    /*
                        Sort the keys before comparing to account for the
                        user pressing the correct keys in a different order
                     */
                    pressed_keys.sort();
                    if keybind.keys == pressed_keys {
                        // Execute the action associated with the keybind
                        keybind.action.execute();
                    }
                }
                // User released a key, remove it from pressed_keys
                self.pressed_keys.retain(|key| key != &key_press.key);
                true // Don't check for WIN here, because it was never pressed
            }
        }
    }
}