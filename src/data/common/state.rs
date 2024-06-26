use crate::data::hook::Hook;
use windows::Win32::Foundation::HWND;

pub struct AppState {
    pub handle: HWND,
    pub hooks: Vec<Box<dyn Hook>>,
}

impl AppState {
    pub fn new(handle: HWND, hooks: Vec<Box<dyn Hook>>) -> Self {
        Self { handle, hooks }
    }
}
