use crate::data::common::state::AppState;
use crate::data::key::Keybind;
use crate::data::monitor::Monitor;
use crate::hooks;
use crate::state::init;
use std::cell::RefCell;
use std::sync::Arc;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageA, TranslateMessage, MSG, WM_NULL};

pub struct StateManager {
    state: AppState,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            state: init::application(),
        }
    }

    pub fn get_handle(&self) -> &HWND {
        &self.state.handle
    }

    pub fn get_keybinds(&self) -> &Vec<Keybind> {
        &self.state.keybinds
    }

    pub fn get_monitors(&self) -> &Vec<Arc<RefCell<Monitor>>> {
        &self.state.monitors
    }

    pub fn get_hooks(&self) -> &Vec<(String, isize)> {
        &self.state.hooks
    }

    pub fn handle_window_events(&self) {
        let mut message: MSG = MSG::default();
        while crate::win_api::window::get_message(&mut message, unsafe { self.state.handle }).into()
        {
            unsafe {
                let _ = TranslateMessage(&message);
            }
            unsafe {
                DispatchMessageA(&message);
            }
            if message.message == WM_NULL {
                hooks::unset_hooks(&self.state.hooks);
            }
        }
    }
}
