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
use crate::{state, win_api};
use log::{debug, error, warn};
use std::collections::HashMap;
use std::process::exit;
use windows::Win32::Foundation::{HWND, RECT};
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
        let mut monitors = state::init::monitors();
        let mut workspaces: Vec<Workspace> = Vec::new();
        let mut groups: Vec<Group> = Vec::new();
        let windows = win_api::window::get_all();
        let mut index = 0;
        monitors.iter_mut().for_each(|monitor| unsafe {
            let mon_left = monitor.device_mode.Anonymous1.Anonymous2.dmPosition.x;
            let mon_top = monitor.device_mode.Anonymous1.Anonymous2.dmPosition.y;
            // let mon_right = monitor.info.rcMonitor.right;
            // let mut mon_bottom = monitor.info.rcMonitor.bottom;
            let mon_width = monitor.device_mode.dmPelsWidth;
            let mon_height = monitor.device_mode.dmPelsHeight;
            let mon_right = mon_left + mon_width as i32;
            let mut mon_bottom = mon_top + mon_height as i32;
            let taskbar_offset = monitor.info.rcMonitor.bottom - monitor.info.rcWork.bottom;
            mon_bottom -= taskbar_offset;
            let mut windows_on_monitor = Vec::new();
            windows.iter().for_each(|window| {
                let hmonitor = win_api::monitor::hmonitor_from_hwnd(window.hwnd);
                if hmonitor == monitor.hmonitor {
                    windows_on_monitor.push(window);
                }
            });
            if mon_width > mon_height {
                windows_on_monitor.sort_by(|window, other_window| {
                    window
                        .rect
                        .left
                        .partial_cmp(&other_window.rect.left)
                        .unwrap()
                });
            } else {
                windows_on_monitor.sort_by(|window, other_window| {
                    window.rect.top.partial_cmp(&other_window.rect.top).unwrap()
                });
            }
            let hwnds_on_monitor = windows_on_monitor
                .into_iter()
                .map(|window| window.hwnd)
                .collect();

            let default_group = Group {
                index,
                split_axis: Axis::VERTICAL,
                rect: RECT {
                    left: mon_left,
                    top: mon_top,
                    right: mon_right,
                    bottom: mon_bottom,
                },
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
            index += 1;
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
        let hwnd = win_api::window::foreground_hwnd();
        if self.group_manager.managed_hwnds().contains(&&hwnd) {
            return self.group_manager.group_for_hwnd(&hwnd);
        }
        let hmonitor = self.current_monitor();
        let workspaces = self.monitor_manager.workspaces_for_monitor(hmonitor);
        let workspace = self.workspace_manager.active_workspace(workspaces);
        let groups = self.workspace_manager.groups_for_workspace(workspace);
        return groups[groups.len() - 1];
    }

    // Separate windows by group, then maximize or set position as needed
    pub fn arrange_windows(&mut self, positions: Vec<(HWND, RECT)>) {
        let mut positions_by_group: HashMap<usize, Vec<(HWND, RECT)>> = HashMap::new();
        for (hwnd, position) in positions {
            let current_group = self.group_manager.group_for_hwnd(&hwnd);
            if positions_by_group.contains_key(&current_group) {
                positions_by_group
                    .get_mut(&current_group)
                    .unwrap()
                    .push((hwnd, position));
            } else {
                positions_by_group.insert(current_group, vec![(hwnd, position)]);
            }
        }
        for (group, group_positions) in positions_by_group {
            let workspace = self.workspace_manager.workspace_for_group(group);
            let groups_on_workspace = self.workspace_manager.groups_for_workspace(workspace);
            self.window_manager.set_positions(&group_positions);
            if group_positions.len() == 1 && groups_on_workspace.len() == 1 {
                self.window_manager.maximize(&group_positions[0].0);
            }
        }
    }

    pub fn add_window(&mut self, hwnd: HWND) {
        let added_window = self.window_manager.add_window(hwnd);
        if !added_window {
            return;
        }
        let new_positions = self.group_manager.add_window(self.current_group(), hwnd);
        self.arrange_windows(new_positions);
    }

    pub fn validate(&mut self) {
        // Ensure that every managed window has a group
        let num_windows = self.window_manager.managed_hwnds(false).len();
        let num_group_hwnds = self.group_manager.num_hwnds();
        if num_windows != num_group_hwnds {
            error!("WindowManager and GroupManager state have drifted apart");
            exit(100);
        }
        let (removed, added) = self.window_manager.validate_windows();
        removed.iter().for_each(|hwnd| {
            self.group_manager.remove_window(*hwnd);
        });
        let current_group = self.current_group();
        added.iter().for_each(|hwnd| {
            self.group_manager.add_window(current_group, *hwnd);
        });
        if added.len() > 0 || removed.len() > 0 {
            warn!("Encountered unmanaged windows during validation, all windows should be added/removed via the event listener");
        }
        // Ensure that every managed window has a group
        let num_windows = self.window_manager.managed_hwnds(false).len();
        let num_group_hwnds = self.group_manager.num_hwnds();
        if num_windows != num_group_hwnds {
            error!("WindowManager and GroupManager state have drifted apart");
            exit(100);
        }
    }
}

impl StateManager {
    fn candidate_hwnds(&self) -> Vec<HWND> {
        let hmonitors = self.monitor_manager.get_all();
        let workspaces: Vec<usize> = hmonitors
            .into_iter()
            .flat_map(|hmonitor| self.monitor_manager.workspaces_for_monitor(hmonitor))
            .cloned()
            .collect();
        let groups: Vec<usize> = workspaces
            .into_iter()
            .flat_map(|workspace| self.workspace_manager.groups_for_workspace(workspace))
            .cloned()
            .collect();
        let mut candidate_hwnds = self.group_manager.hwnds_from_groups(&groups);
        let manageable_hwnds = self.window_manager.managed_hwnds(true);
        candidate_hwnds.retain(|hwnd| manageable_hwnds.contains(&hwnd));
        return candidate_hwnds;
    }

    pub fn focus_window_in_direction(&mut self, direction: Direction) {
        let current_hwnd = win_api::window::foreground_hwnd();
        let candidate_hwnds = self.candidate_hwnds();
        let nearest_hwnd =
            self.window_manager
                .find_nearest_in_direction(current_hwnd, direction, candidate_hwnds);
        if nearest_hwnd.is_some() {
            self.window_manager.focus(nearest_hwnd.unwrap())
        }
    }

    pub fn move_window_in_direction(&mut self, direction: Direction) {
        let current_hwnd = win_api::window::foreground_hwnd();
        // Current group
        let nearest_hwnd_opt = self
            .group_manager
            .candidate_in_direction(&current_hwnd, &direction);
        if nearest_hwnd_opt.is_some() {
            let updated_group = self
                .group_manager
                .swap_windows(current_hwnd, nearest_hwnd_opt.unwrap());
            let manageable_windows = self.window_manager.managed_hwnds(true);
            self.arrange_windows(
                self.group_manager
                    .calculate_window_positions(updated_group, &manageable_windows),
            );
            return;
        }
        // Adjacent workspace group
        let current_group = self.group_manager.group_for_hwnd(&current_hwnd);
        let adjacent_group_opt = self
            .workspace_manager
            .group_in_direction(current_group, &direction);
        if adjacent_group_opt.is_some() {
            let adjacent_group = adjacent_group_opt.unwrap();
            let mut new_positions = self.group_manager.remove_window(current_hwnd);
            new_positions.extend_from_slice(
                self.group_manager
                    .add_window_direction(adjacent_group, &current_hwnd, &direction)
                    .as_slice(),
            );
            self.arrange_windows(new_positions);
            return;
        }
        // Adjacent monitor group
        let current_hmonitor = self.monitor_manager.get_current();
        let nearest_hmonitor_opt = self
            .monitor_manager
            .neighbor_in_direction(&current_hmonitor, &direction);
        if nearest_hmonitor_opt.is_some() {
            debug!("Checking neighboring monitor");
            let mut new_positions = self.group_manager.remove_window(current_hwnd);
            let nearest_hmonitor = nearest_hmonitor_opt.unwrap();
            let workspaces = self
                .monitor_manager
                .workspaces_for_monitor(nearest_hmonitor);
            let workspace = self.workspace_manager.active_workspace(workspaces);
            let groups = self.workspace_manager.groups_for_workspace(workspace);
            let target_group = match direction {
                LEFT | UP => groups[groups.len() - 1],
                DOWN | RIGHT => groups[0],
            };
            new_positions.extend_from_slice(
                self.group_manager
                    .add_window_direction(target_group, &current_hwnd, &direction)
                    .as_slice(),
            );
            self.window_manager.update_dpi(current_hwnd);
            self.arrange_windows(new_positions);
            return;
        }
        error!(
            "Unable to move '{}' {:?}",
            win_api::window::get_window_title(current_hwnd),
            direction
        );
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
            self.window_manager.minimize(&hwnd);
            self.workspace_manager.toggle_active(current_workspace);
        }
        for hwnd in requested_hwnds {
            self.window_manager.restore(&hwnd);
            self.workspace_manager.toggle_active(workspace_id);
        }
    }

    pub fn move_to_workspace(&mut self, workspace_id: usize) {
        let hwnd = win_api::window::foreground_hwnd();
        self.window_manager.minimize(&hwnd);
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
