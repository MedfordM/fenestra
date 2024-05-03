use crate::data::key::Keybind;
use crate::data::monitor::Monitor;
use std::cell::RefCell;
use std::sync::Arc;
use windows::Win32::Foundation::HWND;

pub struct AppState {
    pub handle: HWND,
    pub hooks: Vec<(String, isize)>,
    pub keybinds: Vec<Keybind>,
    pub monitors: Vec<Arc<RefCell<Monitor>>>,
}

impl AppState {
    pub fn new(
        handle: HWND,
        hooks: Vec<(String, isize)>,
        keybinds: Vec<Keybind>,
        monitors: Vec<Arc<RefCell<Monitor>>>,
    ) -> Self {
        Self {
            handle,
            monitors,
            hooks,
            keybinds,
        }
    }
}
