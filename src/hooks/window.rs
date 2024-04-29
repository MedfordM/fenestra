use windows::Win32::UI::WindowsAndMessaging::EVENT_SYSTEM_MOVESIZEEND;
use windows::Win32::{
    Foundation::HWND,
    UI::{
        Accessibility::HWINEVENTHOOK,
        WindowsAndMessaging::{CHILDID_SELF, EVENT_SYSTEM_FOREGROUND, OBJID_WINDOW},
    },
};

use crate::state::MONITORS;
use crate::win_api::monitor::get_monitor_from_window;
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

            let window_result = Window::from(hwnd);
            if window_result.is_none() {
                return;
            }

            let window = window_result.unwrap();
            // TODO: Add a border to the newly focused window here
            // let border_hwnd = create_window(
            //   WS_EX_TOOLWINDOW | WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_NOACTIVATE,
            //   "border",
            //   "border",
            //   WS_POPUP | WS_SYSMENU,
            //   window.rect.left,
            //   window.rect.top,
            //   window.rect.right - window.rect.left,
            //   window.rect.bottom - window.rect.top,
            //   get_main_module()
            // );

            // set_transparent(border_hwnd);

            // debug!("Foreground window was updated: {}", window.title);

            let monitor_handle = get_monitor_from_window(window.hwnd);
            let monitors = &mut MONITORS.lock().unwrap();
            monitors.iter_mut().for_each(|monitor| {
                /*
                   On all monitors, remove any stale references to the current window

                   For the monitor containing the currently focused window,
                   remove the (potentially stale) window state from the owning
                   workspace, then add it to the current workspace.
                */
                monitor.remove_window(&window);
                if monitor.hmonitor == monitor_handle {
                    monitor.add_window(&window);
                }
            });
        }
        EVENT_SYSTEM_MOVESIZEEND => {
            let window_result = Window::from(hwnd);
            if window_result.is_none() {
                return;
            }
            let window = window_result.unwrap();
            let monitor_handle = get_monitor_from_window(window.hwnd);
            let monitors = &mut MONITORS.lock().unwrap();
            monitors.iter_mut().for_each(|monitor| {
                if monitor.hmonitor == monitor_handle {
                    let current_workspace = monitor.current_workspace();
                    if !current_workspace.contains_window(&window) {
                        current_workspace.add_window(&window);
                    }
                    current_workspace.arrange_windows();
                } else if monitor.contains_window(&window) {
                    monitor.remove_window(&window);
                    monitor.current_workspace().arrange_windows();
                }
                // let all_windows = monitor.all_windows();
                // let window_titles: Vec<String> =
                //     all_windows.iter().map(|w| w.title.clone()).collect();
                // debug!("Monitor contains windows {:?}", window_titles);
            });
        }
        _ => (),
    }
}
