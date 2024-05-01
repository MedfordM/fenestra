use log::debug;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::sync::Arc;
use windows::Win32::Foundation::RECT;

use crate::data::common::axis::Axis::VERTICAL;
use windows::Win32::Graphics::Gdi::{DEVMODEA, HMONITOR, MONITORINFO};
use windows::Win32::UI::Shell::Common::DEVICE_SCALE_FACTOR;

use crate::data::common::direction::{Direction, DirectionCandidate};
use crate::data::group::Group;
use crate::data::window::Window;
use crate::data::workspace::Workspace;
use crate::win_api::monitor::{get_monitor, get_windows_on_monitor};

#[derive(Clone, Default)]
pub struct Monitor {
    pub hmonitor: HMONITOR,
    pub name: String,
    pub info: MONITORINFO,
    pub device_mode: DEVMODEA,
    pub scale: DEVICE_SCALE_FACTOR,
    pub workspaces: Vec<Workspace>,
    pub neighbors: HashMap<Direction, Arc<RefCell<Monitor>>>,
}

impl PartialEq for Monitor {
    fn eq(&self, other: &Self) -> bool {
        self.hmonitor == other.hmonitor || self.name == other.name
    }
}

impl Monitor {
    pub fn from(value: HMONITOR) -> Self {
        get_monitor(value)
    }

    pub fn current_workspace(&mut self) -> &mut Workspace {
        return self
            .workspaces
            .iter_mut()
            .find(|workspace| workspace.focused == true)
            .expect("Unable to find current workspace");
    }

    pub fn get_workspace(&mut self, id: u32) -> &mut Workspace {
        return &mut self.workspaces[(id - 1) as usize];
    }

    pub fn focus_workspace(&mut self, id: u32) {
        let current_workspace = self.current_workspace();
        if id == current_workspace.id {
            debug!("Skipping request to focus the current workspace");
            return;
        }
        current_workspace.unfocus();
        self.workspaces[(id - 1) as usize].focus();
    }

    pub fn add_window_to_workspace(&mut self, id: u32, window: &Window) {
        let current_workspace = self.current_workspace();
        if id == current_workspace.id {
            return;
        }
        current_workspace.remove_window(window);
        self.workspaces[(id - 1) as usize].add_window(window);
    }

    pub fn workspace_from_window(&mut self, window: &Window) -> Option<&mut Workspace> {
        if !self.contains_window(window) {
            return None;
        }
        let search_result = self
            .workspaces
            .iter_mut()
            .find(|workspace| workspace.all_windows().contains(window));
        return search_result;
    }

    pub fn init_workspaces(hmonitor: HMONITOR, rect: RECT) -> Vec<Workspace> {
        let mut workspaces: Vec<Workspace> = vec![];
        let default_workspace: Workspace = Workspace {
            id: 1,
            focused: true,
            rect,
            split_axis: VERTICAL,
            groups: vec![Group {
                index: 0,
                windows: get_windows_on_monitor(hmonitor),
                split_axis: VERTICAL,
            }],
        };
        workspaces.push(default_workspace);
        for i in 2..10 {
            let workspace: Workspace = Workspace {
                id: i,
                focused: false,
                rect,
                split_axis: VERTICAL,
                groups: vec![Group {
                    index: 0,
                    windows: HashSet::new(),
                    split_axis: VERTICAL,
                }],
            };
            workspaces.push(workspace);
        }
        return workspaces;
    }

    pub fn contains_window(&self, window: &Window) -> bool {
        return self.all_windows().contains(window);
    }

    pub fn add_window(&mut self, window: &Window) -> bool {
        let current_workspace = self.current_workspace();
        return current_workspace.add_window(window);
    }

    pub fn remove_window(&mut self, window: &Window) -> bool {
        if !self.contains_window(window) {
            return false;
        }
        let workspace = self.current_workspace();
        return workspace.remove_window(window);
    }

    pub fn all_windows(&self) -> HashSet<Window> {
        let mut all_windows: HashSet<Window> = HashSet::new();
        self.workspaces.iter().for_each(|workspace| {
            all_windows.extend(workspace.all_windows().clone());
        });
        return all_windows;
    }

    pub fn create_nearest_candidate(self, direction: &Direction) -> DirectionCandidate<Monitor> {
        let monitor_rect = match direction {
            Direction::LEFT | Direction::RIGHT => RECT {
                left: unsafe { self.device_mode.Anonymous1.Anonymous2 }
                    .dmPosition
                    .x,
                top: 0,
                bottom: 0,
                right: 0,
            },
            Direction::UP | Direction::DOWN => RECT {
                left: 0,
                top: unsafe { self.device_mode.Anonymous1.Anonymous2 }
                    .dmPosition
                    .y,
                bottom: 0,
                right: 0,
            },
        };
        DirectionCandidate {
            name: String::from(&self.name),
            object: self,
            rect: monitor_rect,
            offset_x: None,
            offset_y: None,
        }
    }
}

impl Debug for Monitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {:?}, {:?}, Neighbors: {:?}",
            self.name, self.info.rcWork, self.workspaces, self.neighbors
        )
    }
}
