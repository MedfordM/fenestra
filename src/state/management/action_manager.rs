use crate::data::common::direction::Direction;
use crate::data::common::direction::Direction::{DOWN, LEFT, RIGHT, UP};
use crate::state::management::state_manager::StateManager;
use crate::win_api;

pub struct ActionManager {
    state_manager: StateManager
}

impl ActionManager {
    pub fn new(state_manager: StateManager) -> Self {
        Self { state_manager }
    }
    
    pub fn focus_window_in_direction(&mut self, direction: Direction) {
        let current_hwnd = win_api::window::foreground_hwnd();
        let nearest_hwnd = unsafe { self.state_manager.window_manager.find_nearest_in_direction(current_hwnd, direction)};
        if nearest_hwnd.is_some() {
            unsafe { self.state_manager.window_manager.focus(nearest_hwnd.unwrap()) };
        }
    }

    pub fn move_window_in_direction(&mut self, direction: Direction) {
        let current_hwnd = win_api::window::foreground_hwnd();
        let nearest_result = unsafe { self.state_manager.window_manager.find_nearest_in_direction(current_hwnd, direction.clone()) };
        unsafe {
            let new_positions = match nearest_result {
                Some(nearest_hwnd) => {
                    self.state_manager.group_manager.swap_windows(current_hwnd, nearest_hwnd)
                },
                None => {
                    let nearest_hmonitor_result = self.state_manager.monitor_manager.find_nearest_in_direction(direction.clone());
                    match nearest_hmonitor_result {
                        Some(nearest_hmonitor) => {
                            let monitor_workspaces = self.state_manager.monitor_manager.workspaces_for_monitor(nearest_hmonitor);
                            let workspace = self.state_manager.workspace_manager.active_workspace(monitor_workspaces);
                            let groups = self.state_manager.workspace_manager.groups_for_workspace(workspace);
                            let group = match direction {
                                LEFT | DOWN => groups[groups.len() - 1],
                                UP | RIGHT => groups[0],
                            };
                            self.state_manager.group_manager.add_window(group, current_hwnd)
                        }
                        None => Vec::new(),
                    }
                }
            };
            for (hwnd, position) in new_positions {
                self.state_manager.window_manager.set_position(hwnd, position, 0);
            }
        }
    }
    
    pub fn focus_workspace(&mut self, workspace_id: usize) {
        unsafe {
            let current_workspace = self.state_manager.current_workspace();
            let visible_groups = self.state_manager.workspace_manager.groups_for_workspace(current_workspace);
            let visible_hwnds = self.state_manager.group_manager.hwnds_from_groups(visible_groups);
            let requested_groups = self.state_manager.workspace_manager.groups_for_workspace(workspace_id);
            let requested_hwnds = self.state_manager.group_manager.hwnds_from_groups(requested_groups);
            for hwnd in visible_hwnds {
                self.state_manager.window_manager.minimize(hwnd);
                self.state_manager.workspace_manager.toggle_active(current_workspace);
            }
            for hwnd in requested_hwnds {
                self.state_manager.window_manager.restore(hwnd);
                self.state_manager.workspace_manager.toggle_active(workspace_id);
            }
        }
    }
    
    pub fn move_to_workspace(&mut self, workspace_id: usize) {
        let hwnd = win_api::window::foreground_hwnd();
        unsafe {
            self.state_manager.window_manager.minimize(hwnd);
            let groups = self.state_manager.workspace_manager.groups_for_workspace(workspace_id);
            let group = groups[groups.len() - 1];
            let mut new_positions = Vec::new();
            new_positions.append(&mut self.state_manager.group_manager.remove_window(hwnd));
            new_positions.append(&mut self.state_manager.group_manager.add_window(group, hwnd));
            for (hwnd, position) in new_positions {
                self.state_manager.window_manager.set_position(hwnd, position, 0);
            }
        }
    }
}
