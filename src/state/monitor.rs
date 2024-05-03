use crate::data::common::direction::{Direction, ALL_DIRECTIONS};
use crate::data::monitor::Monitor;
use crate::data::window::Window;
use crate::win_api::monitor::get_hmonitor_from_window;
use crate::win_api::window;
use crate::win_api::window::get_foreground_handle;
use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::Arc;

pub fn current(monitors: Vec<Arc<RefCell<Monitor>>>) -> Arc<RefCell<Monitor>> {
    let window_handle = get_foreground_handle();
    let monitor_handle = get_hmonitor_from_window(window_handle);
    return unsafe {
        Arc::clone(
            monitors
                .iter()
                .find(|monitor_ref| Arc::clone(monitor_ref).borrow().hmonitor == monitor_handle)
                .expect("Unable to get current monitor"),
        )
    };
}
