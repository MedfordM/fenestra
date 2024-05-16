use crate::data::common::event::Event;
use windows::Win32::UI::WindowsAndMessaging::EVENT_SYSTEM_MINIMIZEEND;
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
use crate::win_api;
use crate::win_api::window::send_event_message;

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
        EVENT_SYSTEM_MINIMIZESTART => send_event_message(Event::minimize(hwnd)),
        EVENT_SYSTEM_MINIMIZEEND => send_event_message(Event::restore(hwnd)),
        EVENT_SYSTEM_MOVESIZEEND => send_event_message(Event::move_size(hwnd)),
        EVENT_SYSTEM_FOREGROUND => {
            if hwnd.0 == 0 || object_id == OBJID_WINDOW.0 || child_id != CHILDID_SELF as i32 {
                return;
            }
            let window_result = win_api::window::get_window(hwnd);
            if window_result.is_none() {
                return;
            }
            send_event_message(Event::focus(hwnd));
        }
        _ => (),
    }
}
