use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use windows::Win32::UI::WindowsAndMessaging::{HC_ACTION, WM_KEYUP};
use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{HHOOK, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL},
};

use crate::data::common::event::Event;
use crate::data::hook::Hook;
use crate::data::key::WINDOWS_KEY_CODE;
use crate::win_api::window::send_event_message;
use crate::{
    win_api,
    win_api::hook::{call_next_hook, set_window_hook},
};

pub struct KeyboardHook {
    hook: HHOOK,
}

impl KeyboardHook {
    pub fn new() -> Self {
        Self {
            hook: HHOOK::default(),
        }
    }
}
unsafe impl Send for KeyboardHook {}
unsafe impl Sync for KeyboardHook {}
pub unsafe extern "system" fn callback(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code < 0 || code != HC_ACTION as i32 {
        return call_next_hook(code, w_param, l_param);
    }
    // Post the key code back to the main application message queue
    let hook_struct: *mut KBDLLHOOKSTRUCT = l_param.0 as *mut KBDLLHOOKSTRUCT;
    let keyboard_hook_struct = hook_struct.as_ref().unwrap();
    let key_code = keyboard_hook_struct.vkCode;
    send_event_message(Event::key_event(key_code as isize, w_param));
    // Suppress the Windows key
    let win_pressed = (GetAsyncKeyState(WINDOWS_KEY_CODE) & (1 << 15)) == (1 << 15);
    if win_pressed {
        if key_code != WINDOWS_KEY_CODE as u32 && w_param.0 != WM_KEYUP as usize {
            return LRESULT(1);
        }
    }
    return call_next_hook(code, w_param, l_param);
}

impl Hook for KeyboardHook {
    fn set(&mut self) {
        self.hook = set_window_hook(WH_KEYBOARD_LL, callback);
    }

    fn remove(&mut self) {
        win_api::hook::unset_window_hook(self.hook);
        self.hook = HHOOK::default();
    }
}
