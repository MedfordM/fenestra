use log::debug;
use windows::Win32::{
    Foundation::HWND,
    UI::{
        Accessibility::HWINEVENTHOOK,
        WindowsAndMessaging::{
            CHILDID_SELF, EVENT_SYSTEM_FOREGROUND, EVENT_SYSTEM_MINIMIZESTART,
            EVENT_SYSTEM_MOVESIZEEND, OBJID_WINDOW,
        },
    },
};
use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{EVENT_SYSTEM_MINIMIZEEND, PostMessageA, WM_APP};

use crate::data::hook::Hook;
use crate::win_api;

pub struct EventHook {
    hook: HWINEVENTHOOK,
}

impl EventHook {
    pub fn new() -> Self {
        Self {
            hook: HWINEVENTHOOK::default(),
        }
    }
}

impl Hook for EventHook {
    fn set(&mut self) {
        self.hook = win_api::hook::set_event_hook(callback);
    }

    fn remove(&mut self) {
        win_api::hook::unset_event_hook(self.hook);
        self.hook = HWINEVENTHOOK::default();
    }
}

pub unsafe extern "system" fn callback(
    _: HWINEVENTHOOK,
    event: u32,
    hwnd: HWND,
    object_id: i32,
    child_id: i32,
    _: u32,
    _: u32,
) {
    match event {
        EVENT_SYSTEM_MINIMIZESTART => {
            let window_result = win_api::window::get_window(hwnd);
            if window_result.is_none() {
                return;
            }
            let _ = PostMessageA(None, WM_APP + 3, WPARAM(1), LPARAM(hwnd.0));
        },
        EVENT_SYSTEM_MINIMIZEEND => {
            let window_result = win_api::window::get_window(hwnd);
            if window_result.is_none() {
                return;
            }
            let _ = PostMessageA(None, WM_APP + 3, WPARAM(2), LPARAM(hwnd.0));
        }
        EVENT_SYSTEM_FOREGROUND => {
            if hwnd.0 == 0 {
                return;
            }
            
            if object_id != OBJID_WINDOW.0 {
                return;
            }
            
            if child_id != CHILDID_SELF as i32 {
                return;
            }
            
            let window_result = win_api::window::get_window(hwnd);
            if window_result.is_none() {
                return;
            }
            // debug!("Foreground window was updated: {}", window.title);
            let _ = PostMessageA(None, WM_APP + 3, WPARAM(0), LPARAM(hwnd.0));
        },
        EVENT_SYSTEM_MOVESIZEEND => {
            let window_result = win_api::window::get_window(hwnd);
            if window_result.is_none() {
                return;
            }
            // STATE_MANAGER.borrow_mut().validate();
        },
        _ => (),
    }
}
