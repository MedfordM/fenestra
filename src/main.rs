use crate::data::common::event::{FOCUS, KEY_EVENT, MINIMIZE, RESTORE, WINDOW_EVENT};
use crate::data::key::{Key, KeyEvent, KeyEventType};
use crate::state::management::key_manager::KeyManager;
use crate::state::management::state_manager::StateManager;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{MSG, WM_NULL};

mod actions;
mod config;
mod data;
mod hooks;
mod state;
mod win_api;

fn main() {
    env_logger::init();
    let mut state_manager = StateManager::new();
    state_manager.arrange_all_windows();
    let mut key_manager = KeyManager::new();
    let mut message = MSG::default();
    win_api::window::get_message(&mut message);
    while message.message != WM_NULL {
        win_api::window::get_message(&mut message);
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
                    FOCUS => state_manager.add_window(hwnd),
                    MINIMIZE => {
                        state_manager.window_manager.minimize(&hwnd);
                        let group = state_manager.group_manager.group_for_hwnd(&hwnd);
                        let workspace = state_manager.workspace_manager.workspace_for_group(group);
                        let num_workspace_groups = state_manager
                            .workspace_manager
                            .groups_for_workspace(workspace)
                            .len();
                        let manageable_windows = state_manager.window_manager.managed_hwnds(true);
                        let new_positions = state_manager.group_manager.calculate_window_positions(
                            vec![group],
                            num_workspace_groups,
                            &manageable_windows,
                        );
                        state_manager.arrange_windows(new_positions);
                    }
                    RESTORE => {
                        state_manager.window_manager.restore(&hwnd);
                        let group = state_manager.group_manager.group_for_hwnd(&hwnd);
                        let workspace = state_manager.workspace_manager.workspace_for_group(group);
                        let num_workspace_groups = state_manager
                            .workspace_manager
                            .groups_for_workspace(workspace)
                            .len();
                        let manageable_windows = state_manager.window_manager.managed_hwnds(true);
                        let new_positions = state_manager.group_manager.calculate_window_positions(
                            vec![group],
                            num_workspace_groups,
                            &manageable_windows,
                        );
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
