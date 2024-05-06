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

use crate::data::hook::Hook;
use crate::state::STATE_MANAGER;
use crate::win_api;

pub struct EventHook {
    hook: HWINEVENTHOOK,
}

impl EventHook {
    pub fn new() -> Self {
        Self {
            hook: HWINEVENTHOOK::default()
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
            let new_positions = STATE_MANAGER.group_manager.remove_window(hwnd);
            for (hwnd, position) in new_positions {
                STATE_MANAGER.window_manager.set_position(hwnd, position, 0);
            }
        }
        // EVENT_SYSTEM_MINIMIZEEND => {
        //     let window_result = Window::from(hwnd);
        //     if window_result.is_none() {
        //         return;
        //     }
        //
        //     let window = window_result.unwrap();
        //     let monitor_ref = Monitor::current();
        //     let mut monitor = monitor_ref.borrow_mut();
        //     monitor.add_window(window);
        //     monitor.current_workspace().arrange_windows();
        // }
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
            STATE_MANAGER.add_window(hwnd);
            STATE_MANAGER.validate();
        }
        EVENT_SYSTEM_MOVESIZEEND => {
            let window_result = win_api::window::get_window(hwnd);
            if window_result.is_none() {
                return;
            }
            STATE_MANAGER.validate();
        }
        _ => (),
    }
}
