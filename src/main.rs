use crate::data::common::event::{
    CREATE, DESTROY, FOCUS, KEY_EVENT, MINIMIZE, MOVE_SIZE, RESTORE, WINDOW_EVENT,
};
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
                    MINIMIZE => {
                        state_manager.window_manager.minimize(&hwnd);
                        let group = state_manager.group_manager.group_for_hwnd(&hwnd);
                        let manageable_windows = state_manager.window_manager.managed_hwnds(true);
                        let new_positions = state_manager
                            .group_manager
                            .calculate_window_positions(vec![group], &manageable_windows);
                        state_manager.arrange_windows(new_positions);
                    }
                    RESTORE => {
                        state_manager.window_manager.restore(&hwnd);
                        let group = state_manager.group_manager.group_for_hwnd(&hwnd);
                        let manageable_windows = state_manager.window_manager.managed_hwnds(true);
                        let new_positions = state_manager
                            .group_manager
                            .calculate_window_positions(vec![group], &manageable_windows);
                        state_manager.arrange_windows(new_positions);
                    }
                    // Update application state when the user manually moves a window
                    MOVE_SIZE => {
                        let old_group = state_manager.group_manager.group_for_hwnd(&hwnd);
                        let mut updated_groups = vec![old_group];
                        let old_workspace = state_manager
                            .workspace_manager
                            .workspace_for_group(old_group);
                        let old_hmonitor = state_manager
                            .monitor_manager
                            .monitor_from_workspace(old_workspace);
                        let new_hmonitor = state_manager.monitor_manager.monitor_from_hwnd(&hwnd);
                        if old_hmonitor != new_hmonitor {
                            state_manager.group_manager.remove_window(&hwnd);
                            let new_workspaces = state_manager
                                .monitor_manager
                                .workspaces_for_monitor(new_hmonitor);
                            let new_workspace = state_manager
                                .workspace_manager
                                .active_workspace(new_workspaces);
                            let new_groups = state_manager
                                .workspace_manager
                                .groups_for_workspace(new_workspace);
                            // TODO: Compute this index based on the direction the window came from
                            let new_group = new_groups[new_groups.len() - 1];
                            updated_groups.push(new_group);
                            state_manager.group_manager.add_window(new_group, hwnd);
                        }
                        let manageable_windows = state_manager.window_manager.managed_hwnds(true);
                        let new_positions = state_manager
                            .group_manager
                            .calculate_window_positions(updated_groups, &manageable_windows);
                        state_manager.arrange_windows(new_positions);
                    }
                    FOCUS | CREATE => state_manager.add_window(hwnd),
                    DESTROY => state_manager.remove_window(hwnd),
                    _ => {}
                }
                state_manager.validate();
            }
            _ => (),
        }
    }
    hooks::unset_hooks(state_manager.hooks());
}
