pub mod keyboard_hook {
    use std::collections::HashSet;
    use std::sync::Mutex;

    use lazy_static::lazy_static;
    use windows::Win32::{
        Foundation::{LPARAM, LRESULT, WPARAM},
        UI::WindowsAndMessaging::KBDLLHOOKSTRUCT,
    };
    use windows::Win32::UI::WindowsAndMessaging::CallNextHookEx;

    use crate::data::actions::Execute;
    use crate::data::key::{Key, KEY_WINDOWS, KeyAction, Keybind, KeyPress};
    use crate::state::KEYBINDS;

    lazy_static! {
        static ref KEY_COMBO: Mutex<HashSet<Key>> = Mutex::new(HashSet::new());
    }

    pub unsafe extern "system" fn callback(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        if code < 0 {
            return unsafe { CallNextHookEx(None, code, w_param, l_param) };
        }
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
                    return unsafe { CallNextHookEx(None, code, w_param, l_param) };
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
                    return unsafe { CallNextHookEx(None, code, w_param, l_param) };
                }
                // User pressed a defined keybind, mark the key as released and execute the action
                let bind: &Keybind = KEYBINDS.get(bind_index.unwrap()).unwrap();
                KEY_COMBO.lock().unwrap().remove(&key_press.key);
                bind.action.execute();
            }
        }

        // Suppress every instance of the WIN key
        if key_code == KEY_WINDOWS {
            LRESULT(10)
        } else {
            unsafe { CallNextHookEx(None, code, w_param, l_param) }
        }
    }
}
