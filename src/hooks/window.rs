use std::sync::Arc;
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

use crate::state::MONITORS;
use crate::win_api::monitor::{
    ensure_unique_window_state, get_monitor_from_window, revalidate_windows,
};
use crate::{data::window::Window, win_api::hook::set_event_hook};

pub fn init_hook() -> HWINEVENTHOOK {
    return set_event_hook(callback);
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
            let window_result = Window::from(hwnd);
            if window_result.is_none() {
                return;
            }

            let window = window_result.unwrap();
            /*
               In the case that the user manually minimized the window (not using fenestra),
               the now-current window is the one that had focus before the minimized window.

               Because of this, the now-current monitor is NOT guaranteed to be the one that
               the user minimized the window on, making it necessary to manually find the
               monitor that contains the window that initiated this event
            */
            let monitor_ref = Arc::clone(
                MONITORS
                    .iter()
                    .find(|monitor| monitor.borrow().contains_window(&window))
                    .unwrap(),
            );
            let mut monitor = monitor_ref.borrow_mut();
            // If the user minimized the window manually, ignore it until it is focused again
            if monitor.current_workspace().contains_window(&window) {
                // debug!("Removing minimized window '{}' from state", window.title);
                monitor.remove_window(&window);
                monitor.current_workspace().arrange_windows();
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
            // TODO: Add a border to the newly focused window here
            if hwnd.0 == 0 {
                return;
            }

            if object_id != OBJID_WINDOW.0 {
                return;
            }

            if child_id != CHILDID_SELF as i32 {
                return;
            }

            let window_result = Window::from(hwnd);
            if window_result.is_none() {
                return;
            }
            // debug!("Foreground window was updated: {}", window.title);
            revalidate_windows();
            let window = window_result.unwrap();
            let current_monitor_hmonitor = get_monitor_from_window(hwnd);
            ensure_unique_window_state(window, current_monitor_hmonitor);
        }
        EVENT_SYSTEM_MOVESIZEEND => {
            let window_result = Window::from(hwnd);
            if window_result.is_none() {
                return;
            }
            revalidate_windows();
            let window = window_result.unwrap();
            let current_monitor_hmonitor = get_monitor_from_window(hwnd);
            ensure_unique_window_state(window, current_monitor_hmonitor);
        }
        _ => (),
    }
}
