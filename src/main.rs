use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageA, MSG, TranslateMessage, WM_NULL};
use crate::state::management::action_manager::ActionManager;
use crate::state::management::state_manager::StateManager;

mod actions;
mod config;
mod data;
mod hooks;
mod state;
mod win_api;

fn main() {
    env_logger::init();
    let state_manager = StateManager::new();
    let action_manager = ActionManager::new(state_manager);
    
    let mut message: MSG = MSG::default();
    while win_api::window::get_message(&mut message, unsafe { state_manager.handle() }).into()
    {
        unsafe {
            let _ = TranslateMessage(&message);
        }
        unsafe {
            DispatchMessageA(&message);
        }
        if message.message == WM_NULL {
            hooks::unset_hooks();
        }
    }
}
