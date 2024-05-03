/*
   On all monitors, remove any stale references to the current window

   For the monitor containing the currently focused window,
   remove the (potentially stale) window state from the owning
   workspace, then add it to the current workspace.
*/
use crate::data::monitor::Monitor;
use crate::data::window::Window;
use log::debug;
use std::cell::RefCell;
use std::sync::Arc;
use windows::Win32::Graphics::Gdi::HMONITOR;

pub fn revalidate_windows(monitors: Vec<Arc<RefCell<Monitor>>>) {
    monitors
        .iter()
        .map(|monitor| Arc::clone(monitor))
        .for_each(|monitor_ref| {
            let mut monitor = monitor_ref.borrow_mut();
            monitor.all_windows().iter().for_each(|window| {
                if Window::from(window.hwnd).is_none() {
                    debug!(
                        "Removing invalid window '{}' from state, as it no longer exists",
                        window.title
                    );
                    monitor.remove_window(window);
                }
            });
        });
}

/*
   Remove all references to a window on all monitors other than the specified current_hmonitor
*/
pub fn ensure_unique_window_state(
    window: Window,
    current_hmonitor: HMONITOR,
    monitors: Vec<Arc<RefCell<Monitor>>>,
) {
    unsafe {
        monitors
            .iter()
            .map(|monitor| Arc::clone(monitor))
            .filter(|monitor| monitor.borrow().hmonitor != current_hmonitor)
            .for_each(|monitor_ref| {
                let mut monitor = monitor_ref.borrow_mut();
                if monitor.contains_window(&window) {
                    monitor.remove_window(&window);
                    monitor.current_workspace().arrange_windows();
                }
            });
    }
    let monitor_ref = Monitor::current();
    let mut monitor = monitor_ref.borrow_mut();
    monitor.add_window(window);
    monitor.current_workspace().arrange_windows();
}
