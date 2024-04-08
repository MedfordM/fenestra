pub mod keyboard_hook {
    use windows::Win32::{Foundation::{LPARAM, LRESULT, WPARAM}, UI::WindowsAndMessaging::KBDLLHOOKSTRUCT};
    use crate::data::key::{Key, KEY_WINDOWS, KeyAction, Keybind, KeyPress};
    use crate::state::APP_STATE;

    static mut KEY_COMBO: Vec<Key> = Vec::new();
    static KEYBINDS: Vec<Keybind> = APP_STATE.keybinds;

    pub unsafe extern "system" fn callback(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        if code < 0 {
            return LRESULT::default();
        }

        let hook_struct: *mut KBDLLHOOKSTRUCT = l_param.0 as *mut KBDLLHOOKSTRUCT;
        let key_code: i32 = hook_struct.as_mut().unwrap().vkCode as i32;

        let key_action: KeyAction = KeyAction::from(w_param.0);
        let key: Key = Key::from(key_code);
        let key_press: KeyPress = KeyPress::new(key_action, key);

        match key_action {
            KeyAction::DOWN => {
                if !KEY_COMBO.contains(&key_press.key) {
                    KEY_COMBO.push(key_press.key.clone());
                }
            },
            KeyAction::UP => {
                if KEY_COMBO.len() == 0 || !KEY_COMBO.contains(&key_press.key) {
                    return LRESULT::default();
                }

                // if !MODIFIER_KEYS.contains(&key_code) {
                //     println!("Read key combo: {:?}", KEY_COMBO);
                // }
                KEY_COMBO.remove(KEY_COMBO.iter().position(|x| x.code == key_press.key.code).unwrap());
            }
        }

        if key_code == KEY_WINDOWS || KEY_COMBO.contains(&Key::from(KEY_WINDOWS)) {
            LRESULT(10)
        } else {
            LRESULT::default()
        }
    }
}