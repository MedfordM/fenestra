use crate::data::common::axis::Axis;
use crate::data::common::direction::Direction;
use crate::data::common::direction::Direction::{DOWN, LEFT, RIGHT, UP};
use crate::data::common::state::AppState;
use crate::data::group::Group;
use crate::data::hook::Hook;
use crate::data::workspace::Workspace;
use crate::state::init;
use crate::state::management::group_manager::GroupManager;
use crate::state::management::monitor_manager::MonitorManager;
use crate::state::management::window_manager::WindowManager;
use crate::state::management::workspace_manager::WorkspaceManager;
use crate::win_api;
use log::warn;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::HMONITOR;

pub struct StateManager {
    state: AppState,
    pub window_manager: WindowManager,
    pub group_manager: GroupManager,
    pub workspace_manager: WorkspaceManager,
    pub monitor_manager: MonitorManager,
}

impl StateManager {
    pub fn new() -> Self {
        let mut monitors = win_api::monitor::get_all();
        let mut workspaces: Vec<Workspace> = Vec::new();
        let mut groups: Vec<Group> = Vec::new();
        let windows = win_api::window::get_all();
        let all_hwnds: Vec<HWND> = windows.iter().map(|window| window.hwnd).collect();
        let index = 0;
        monitors.iter_mut().for_each(|monitor| {
            let mut hwnds_on_monitor = Vec::new();
            all_hwnds.iter().for_each(|hwnd| {
                let hmonitor = win_api::monitor::hmonitor_from_hwnd(*hwnd);
                if hmonitor == monitor.hmonitor {
                    hwnds_on_monitor.push(*hwnd);
                }
            });
            let default_group = Group {
                index,
                split_axis: Axis::VERTICAL,
                rect: monitor.info.rcWork,
                windows: hwnds_on_monitor,
            };
            let default_workspace = Workspace {
                index,
                groups: vec![index],
                active: true,
            };
            monitor.workspaces.push(index);
            workspaces.push(default_workspace);
            groups.push(default_group);
        });
        let monitor_manager = MonitorManager::new(monitors);
        let workspace_manager = WorkspaceManager::new(workspaces);
        let group_manager = GroupManager::new(groups);
        let window_manager = WindowManager::new(windows);
        Self {
            state: init::application(),
            window_manager,
            group_manager,
            workspace_manager,
            monitor_manager,
        }
    }

    pub fn handle(&self) -> HWND {
        self.state.handle.clone()
    }

    pub fn hooks(&mut self) -> &mut Vec<Box<dyn Hook>> {
        &mut self.state.hooks
    }

    pub fn current_monitor(&self) -> HMONITOR {
        self.monitor_manager.get_current()
    }

    pub fn current_workspace(&self) -> usize {
        self.workspace_manager
            .workspace_for_group(self.current_group())
    }

    pub fn current_group(&self) -> usize {
        self.group_manager
            .group_for_hwnd(win_api::window::foreground_hwnd())
    }

    pub fn add_window(&mut self, hwnd: HWND) {
        self.group_manager.add_window(self.current_group(), hwnd);
    }

    pub fn remove_window(&mut self, hwnd: HWND) {
        self.group_manager.remove_window(hwnd);
    }

    pub fn validate(&mut self) {
        let (removed, added) = self.window_manager.validate_windows();
        let mut new_positions = Vec::new();
        removed.into_iter().for_each(|hwnd| {
            new_positions.append(&mut self.group_manager.remove_window(hwnd));
        });
        // This should never happen, new windows should get added by the event listener
        let group = self.current_group();
        added.into_iter().for_each(|hwnd| {
            warn!("Encountered unmanaged windows during validation, all windows should be added via the event listener");
            new_positions.append(&mut self.group_manager.add_window(group, hwnd));
        });
        new_positions.append(&mut self.group_manager.validate());
        for (hwnd, position) in new_positions {
            self.window_manager.set_position(hwnd, position, 0);
        }
    }
}

impl StateManager {
    pub fn focus_window_in_direction(&mut self, direction: Direction) {
        let current_hwnd = win_api::window::foreground_hwnd();
        let nearest_hwnd = self
            .window_manager
            .find_nearest_in_direction(current_hwnd, direction);
        if nearest_hwnd.is_some() {
            self.window_manager.focus(nearest_hwnd.unwrap())
        }
    }

    pub fn move_window_in_direction(&mut self, direction: Direction) {
        let current_hwnd = win_api::window::foreground_hwnd();
        let nearest_result = self
            .window_manager
            .find_nearest_in_direction(current_hwnd, direction.clone());
        let new_positions = match nearest_result {
            Some(nearest_hwnd) => self.group_manager.swap_windows(current_hwnd, nearest_hwnd),
            None => {
                let nearest_hmonitor_result = self
                    .monitor_manager
                    .find_nearest_in_direction(direction.clone());
                match nearest_hmonitor_result {
                    Some(nearest_hmonitor) => {
                        let monitor_workspaces = self
                            .monitor_manager
                            .workspaces_for_monitor(nearest_hmonitor);
                        let workspace = self.workspace_manager.active_workspace(monitor_workspaces);
                        let groups = self.workspace_manager.groups_for_workspace(workspace);
                        let group = match direction {
                            LEFT | DOWN => groups[groups.len() - 1],
                            UP | RIGHT => groups[0],
                        };
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

    pub fn focus_workspace(&mut self, workspace_id: usize) {
        let current_workspace = self.current_workspace();
        let visible_groups = self
            .workspace_manager
            .groups_for_workspace(current_workspace);
        let visible_hwnds = self.group_manager.hwnds_from_groups(visible_groups);
        let requested_groups = self.workspace_manager.groups_for_workspace(workspace_id);
        let requested_hwnds = self.group_manager.hwnds_from_groups(requested_groups);
        for hwnd in visible_hwnds {
            self.window_manager.minimize(hwnd);
            self.workspace_manager.toggle_active(current_workspace);
        }
        for hwnd in requested_hwnds {
            self.window_manager.restore(hwnd);
            self.workspace_manager.toggle_active(workspace_id);
        }
    }

    pub fn move_to_workspace(&mut self, workspace_id: usize) {
        let hwnd = win_api::window::foreground_hwnd();
        self.window_manager.minimize(hwnd);
        let groups = self.workspace_manager.groups_for_workspace(workspace_id);
        let group = groups[groups.len() - 1];
        let mut new_positions = Vec::new();
        new_positions.append(&mut self.group_manager.remove_window(hwnd));
        new_positions.append(&mut self.group_manager.add_window(group, hwnd));
        for (hwnd, position) in new_positions {
            self.window_manager.set_position(hwnd, position, 0);
        }
    }
}
