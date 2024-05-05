use crate::data::common::direction::Direction;
use crate::state::management::group_manager::GroupManager;
use crate::state::management::monitor_manager::MonitorManager;
use crate::state::management::window_manager::WindowManager;
use crate::state::management::workspace_manager::WorkspaceManager;
use crate::win_api;

pub struct ActionManager {
    window_manager: WindowManager,
    group_manager: GroupManager,
    workspace_manager: WorkspaceManager,
    monitor_manager: MonitorManager
}

impl ActionManager {
    pub fn focus_window_in_direction(&mut self, direction: Direction) {
        let current_hwnd = win_api::window::foreground_hwnd();
        let nearest_hwnd = self.window_manager.find_nearest_in_direction(current_hwnd, direction);
        if nearest_hwnd.is_some() {
            self.window_manager.focus(nearest_hwnd.unwrap());
        }       
    }
    
    pub fn move_window_in_direction(&mut self, direction: Direction) {
        let current_hwnd = win_api::window::foreground_hwnd();
        let nearest_result = self.window_manager.find_nearest_in_direction(current_hwnd, direction.clone());
        let new_positions = match nearest_result {
            Some(nearest_hwnd) => {
                self.group_manager.swap_windows(current_hwnd, nearest_hwnd)
            },
            None => {
                let nearest_hmonitor_result = self.monitor_manager.find_nearest_in_direction(direction.clone());
                match nearest_hmonitor_result {
                    Some(nearest_hmonitor) => {
                        let monitor_workspaces = self.monitor_manager.workspaces_for_monitor(nearest_hmonitor);
                        let workspace = self.workspace_manager.get_current_workspace(monitor_workspaces);
                        let groups = self.workspace_manager.groups_for_workspace(workspace);
                        let group = self.workspace_manager.get_current_workspace(groups);
                        self.group_manager.add_window(group, current_hwnd)
                    }
                    None => Vec::new(),
                }
            }
        };
        for (hwnd, position) in new_positions {
            self.window_manager.set_position(hwnd, position, 0);
        }
    }
}
