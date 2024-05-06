use log::debug;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageA, PostMessageA, SendMessageA, TranslateMessage, HWND_BROADCAST, MSG, WM_APP,
};
use windows::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{HHOOK, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL},
};

use crate::data::hook::Hook;
use crate::data::key::WINDOWS_KEY_CODE;
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

pub unsafe extern "system" fn callback(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let hook_struct: *mut KBDLLHOOKSTRUCT = l_param.0 as *mut KBDLLHOOKSTRUCT;
    let key_code: i32 = hook_struct.as_ref().unwrap().vkCode as i32;
    let _ = PostMessageA(None, WM_APP + 2, w_param, LPARAM(key_code as isize));
    match key_code {
        WINDOWS_KEY_CODE => LRESULT(10),
        _ => call_next_hook(code, w_param, l_param),
    }
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
