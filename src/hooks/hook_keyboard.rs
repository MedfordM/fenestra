pub mod keyboard_hook {
    use log::debug;
    use windows::Win32::{
        Foundation::{LPARAM, LRESULT, WPARAM},
        UI::WindowsAndMessaging::KBDLLHOOKSTRUCT,
    };

    use crate::data::action::Execute;
    use crate::data::key::{Key, KeyAction, Keybind, KeyPress, WINDOWS_KEY_CODE};
    use crate::state::KEYBINDS;
    use crate::state::PRESSED_KEYS;
    use crate::win_api::misc::call_next_hook;

    pub unsafe extern "system" fn callback(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        let hook_struct: *mut KBDLLHOOKSTRUCT = l_param.0 as *mut KBDLLHOOKSTRUCT;
        let key_code: i32 = hook_struct.as_mut().unwrap().vkCode as i32;

        let key_action: KeyAction = KeyAction::from(w_param.0);
        let key: Key = Key::from(key_code);
        let key_press: KeyPress = KeyPress::new(key_action, key);
        let mut pressed_keys = PRESSED_KEYS.lock().unwrap();

        match key_action {
            KeyAction::DOWN => {
                // User pressed a key, add it to KEY_COMBO
                if !&pressed_keys.contains(&key_press.key) {
                    pressed_keys.push(key_press.key.clone());
                }
            }
            KeyAction::UP => {
                pressed_keys.sort();
                // debug!("Evaluating key combo: {:?}", &pressed_keys);
                let bind_index = &KEYBINDS.iter().position(|key_bind| {
                    // Attempt to find a keybind that matches the pressed_combo
                    let mut bind_keys: Vec<Key> = key_bind.keys.clone();
                    bind_keys.sort();
                    if bind_keys == *pressed_keys {
                        return true;
                    }
                    return false;
                });
                // if bind_index.is_none() {
                //     /*
                //        Attempt to find a keybind that transitively matches the pressed_combo:
                //        It's important that this checks for partial equality so that the keys can
                //        be pressed in any order and still match the defined keybind -
                //
                //        For example, if a user has an action defined as 'CTRL + ALT + H',
                //        'ALT + CTRL + H' would count as a match as well, and execute the action
                //     */
                //     bind_index = &KEYBINDS.iter().position(|key_bind| {
                //         if key_bind.keys.iter().all(|k| pressed_keys.contains(k)) {
                //             return true;
                //         };
                //         return false;
                //     })
                // }
                if bind_index.is_some() {
                    // User pressed a defined keybind, execute the action
                    let bind: &Keybind = &KEYBINDS.get(bind_index.unwrap()).unwrap();
                    debug!("Executing action for keybind {:?}", bind.keys);
                    bind.action.execute();
                }
                // Mark the key as released and carry on
                let key_index = &pressed_keys
                    .iter()
                    .position(|k| k == &key_press.key)
                    .unwrap();
                pressed_keys.remove(*key_index);
            }
        }
        // Suppress every instance of the WIN key
        // TODO: Instead, check for any key in $modifier
        let win_key: Key = Key::from(WINDOWS_KEY_CODE);
        if key_press.key == win_key || pressed_keys.contains(&win_key) {
            LRESULT(10)
        } else {
            return call_next_hook(code, w_param, l_param);
        }
    }
}
