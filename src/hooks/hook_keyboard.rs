pub mod keyboard_hook {
    use crate::data::key::{Key, KeyAction, KeyPress, Keybind, KEY_WINDOWS};
    use crate::state::KEYBINDS;
    use lazy_static::lazy_static;
    use std::collections::HashSet;
    use std::sync::Mutex;
    use windows::Win32::{
        Foundation::{LPARAM, LRESULT, WPARAM},
        UI::WindowsAndMessaging::KBDLLHOOKSTRUCT,
    };

    lazy_static! {
        static ref KEY_COMBO: Mutex<HashSet<Key>> = Mutex::new(HashSet::new());
    }

    pub unsafe extern "system" fn callback(
        _code: i32,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        let hook_struct: *mut KBDLLHOOKSTRUCT = l_param.0 as *mut KBDLLHOOKSTRUCT;
        let key_code: i32 = hook_struct.as_mut().unwrap().vkCode as i32;

        let key_action: KeyAction = KeyAction::from(w_param.0);
        let key: Key = Key::from(key_code);
        let key_press: KeyPress = KeyPress::new(key_action, key);

        match key_action {
            KeyAction::DOWN => {
                // User pressed a key, add it to KEY_COMBO
                KEY_COMBO.lock().unwrap().insert(key_press.key.clone());
            }
            KeyAction::UP => {
                /*
                   Sanity check:
                   - Have we pressed any keys down?
                   - Were any of those keys the released key?
                */
                if KEY_COMBO.lock().unwrap().len() == 0
                    || !KEY_COMBO.lock().unwrap().contains(&key_press.key)
                {
                    return LRESULT::default();
                }
                /*
                   Attempt to find a keybind that transitively matches the pressed_combo:
                   It's important that this checks for partial equality so that the keys can
                   be pressed in any order and still match the defined keybind -

                   For example, if a user has an action defined as 'CTRL + ALT + H',
                   'ALT + CTRL + H' would count as a match as well, and execute the action
                */
                let bind_index = &KEYBINDS.iter().position(|key_bind| {
                    key_bind
                        .keys
                        .iter()
                        .all(|key| KEY_COMBO.lock().unwrap().contains(key))
                });
                if bind_index.is_none() {
                    // No matching keybind, mark the key as released and carry on
                    KEY_COMBO.lock().unwrap().remove(&key_press.key);
                    return LRESULT::default();
                }
                // User pressed a defined keybind, mark the key as released and execute the action
                let bind: &Keybind = KEYBINDS.get(bind_index.unwrap()).unwrap();
                println!(
                    "Read key combo: {:?}, executing associated action: {:?}",
                    bind.keys, bind.action
                );
                KEY_COMBO.lock().unwrap().remove(&key_press.key);
            }
        }

        // Suppress every instance of the WIN key
        if key_code == KEY_WINDOWS {
            LRESULT(10)
        } else {
            LRESULT::default()
        }
    }
}
