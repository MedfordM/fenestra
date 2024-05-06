use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{HHOOK, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL},
};

use crate::{data::key::{Key, KeyAction, KeyPress}, win_api, win_api::hook::{call_next_hook, set_window_hook}};
use crate::data::hook::Hook;
use crate::state::management::state_manager::StateManager;

pub struct KeyboardHook {
    hook: HHOOK,
    state_manager: &mut StateManager,
    pub callback: unsafe extern "system" fn(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT
}

impl KeyboardHook {
    pub fn new(state_manager: &mut StateManager) -> Self {
        Self {
            hook: HHOOK::default(),
        }
    }
    unsafe extern "system" fn callback(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        let hook_struct: *mut KBDLLHOOKSTRUCT = l_param.0 as *mut KBDLLHOOKSTRUCT;
        let key_code: i32 = hook_struct.as_ref().unwrap().vkCode as i32;
        let key_action: KeyAction = KeyAction::from(w_param.0);
        let key: Key = Key::from(key_code);
        let key_press: KeyPress = KeyPress::new(key_action, key);
        let propagate_key_press = self.state_manager.key_manager.handle_keypress(key_press);
        match propagate_key_press {
            true => call_next_hook(code, w_param, l_param),
            false => LRESULT(10)
        }
    }
}

impl Hook for KeyboardHook {
    fn set(&mut self) {
        self.hook = set_window_hook(WH_KEYBOARD_LL, self.callback);
    }

    fn remove(&mut self) {
        win_api::hook::unset_window_hook(self.hook);
        self.hook = HHOOK::default();
    }
}

