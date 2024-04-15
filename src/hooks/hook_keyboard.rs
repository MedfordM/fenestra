pub mod keyboard_hook {
    use log::debug;
    use windows::Win32::{
        Foundation::{LPARAM, LRESULT, WPARAM},
        UI::WindowsAndMessaging::KBDLLHOOKSTRUCT,
    };

    use crate::data::action::Execute;
    use crate::data::key::{Key, KEY_WINDOWS, KeyAction, Keybind, KeyPress};
    use crate::state::KEYBINDS;
    use crate::state::PRESSED_KEYS;
    use crate::win_api::misc::call_next_hook;

    pub unsafe extern "system" fn callback(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        let hook_struct: *mut KBDLLHOOKSTRUCT = l_param.0 as *mut KBDLLHOOKSTRUCT;
        let key_code: i32 = hook_struct.as_mut().unwrap().vkCode as i32;

        let key_action: KeyAction = KeyAction::from(w_param.0);
        let key: Key = Key::from(key_code);
        let key_press: KeyPress = KeyPress::new(key_action, key);

        match key_action {
            KeyAction::DOWN => {
                // User pressed a key, add it to KEY_COMBO
                PRESSED_KEYS.lock().unwrap().insert(key_press.key.clone());
            }
            KeyAction::UP => {
                let mut pressed_keys = PRESSED_KEYS.lock().unwrap();
                let bind_index = &KEYBINDS.iter().position(|key_bind| {
                    // Attempt to find a keybind that matches the pressed_combo
                    if pressed_keys.eq(&key_bind.keys) {
                        return true;
                    }
                    /*
                       Attempt to find a keybind that transitively matches the pressed_combo:
                       It's important that this checks for partial equality so that the keys can
                       be pressed in any order and still match the defined keybind -
    
                       For example, if a user has an action defined as 'CTRL + ALT + H',
                       'ALT + CTRL + H' would count as a match as well, and execute the action
                    */
                    if key_bind.keys.iter().all(|k| pressed_keys.contains(k)) {
                        return true;
                    };
                    return false;
                });
                if bind_index.is_some() {
                    // User pressed a defined keybind, execute the action
                    let bind: &Keybind = KEYBINDS.get(bind_index.unwrap()).unwrap();
                    debug!("Executing action for keybind {:?}", bind.keys);
                    bind.action.execute();
                }
                // Mark the key as released and carry on
                pressed_keys.remove(&key_press.key);
            }
        }
        // Suppress every instance of the WIN key
        // TODO: Instead, check for any key in $modifier
        if KEY_WINDOWS == key_press.key.code || PRESSED_KEYS.lock().unwrap().contains(&Key::from(KEY_WINDOWS)) {
            LRESULT(10)
        } else {
            return call_next_hook(code, w_param, l_param);
        }
    }
}
