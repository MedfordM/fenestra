use crate::data::key::{Key, KeyEvent, KeyEventType};
use crate::state::management::key_manager::KeyManager;
use crate::state::management::state_manager::StateManager;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{MSG, WM_APP, WM_NULL};

mod actions;
mod config;
mod data;
mod hooks;
mod state;
mod win_api;

fn main() {
    env_logger::init();
    let mut state_manager = StateManager::new();
    let mut key_manager = KeyManager::new();
    let mut message = MSG::default();
    let _ = win_api::window::get_message(&mut message);
    const KEY_EVENT: u32 = WM_APP + 2;
    const WINDOW_EVENT: u32 = WM_APP + 3;
    while message.message != WM_NULL {
        let _ = win_api::window::get_message(&mut message);
        match message.message {
            KEY_EVENT => {
                let key = Key::from(message.lParam.0 as i32);
                let event_type = KeyEventType::from(message.wParam.0);
                let key_event = KeyEvent::new(event_type, key);
                key_manager.handle_keypress(key_event, &mut state_manager);
            }
            WINDOW_EVENT => {
                let hwnd = HWND(message.lParam.0);
                if state_manager.ignore_events {
                    continue;
                }
                match message.wParam.0 {
                    0 => {
                        state_manager.add_window(hwnd);
                    }
                    1 => {
                        state_manager.window_manager.minimize(&hwnd);
                        let group = state_manager.group_manager.group_for_hwnd(&hwnd);
                        let manageable_windows = state_manager.window_manager.managed_hwnds(true);
                        let new_positions = state_manager
                            .group_manager
                            .calculate_window_positions(vec![group], &manageable_windows);
                        state_manager.arrange_windows(new_positions);
                    }
                    2 => {
                        state_manager.window_manager.restore(&hwnd);
                        let group = state_manager.group_manager.group_for_hwnd(&hwnd);
                        let manageable_windows = state_manager.window_manager.managed_hwnds(true);
                        let new_positions = state_manager
                            .group_manager
                            .calculate_window_positions(vec![group], &manageable_windows);
                        state_manager.arrange_windows(new_positions);
                    }
                    _ => {}
                }
                state_manager.validate();
            }
            _ => (),
        }
    }
    hooks::unset_hooks(state_manager.hooks());
}
