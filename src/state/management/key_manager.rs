use crate::data::action::Action;
use crate::data::key::KeyEventType::{PRESS, RELEASE};
use crate::data::key::{Key, KeyEvent, Keybind};
use crate::state::init;
use crate::state::management::state_manager::StateManager;

pub struct KeyManager {
    keybinds: Vec<Keybind>,
    pressed_keys: Vec<Key>,
}

impl KeyManager {
    pub fn new() -> Self {
        Self {
            keybinds: init::keybinds(),
            pressed_keys: Vec::new(),
        }
    }

    // Returns a bool based on if the keypress should be propagated to the rest of the key listeners
    pub fn handle_keypress(&mut self, key_press: KeyEvent, state_manager: &mut StateManager) {
        match key_press.event {
            PRESS => {
                // User pressed a key, add it to pressed_keys
                if !self.pressed_keys.contains(&key_press.key) {
                    self.pressed_keys.push(Key::from(key_press.key.code));
                }
            }
            RELEASE => {
                // Attempt to find a matching keybind
                for keybind in self.keybinds.iter() {
                    /*
                       Sort the keys before comparing to account for the
                       user pressing the correct keys in a different order
                    */
                    self.pressed_keys.sort();
                    if keybind.keys == *self.pressed_keys {
                        // Execute the action associated with the keybind
                        keybind.action.execute(state_manager);
                    }
                }
                // User released a key, remove it from pressed_keys
                self.pressed_keys.retain(|key| key != &key_press.key);
            }
        }
    }
}
