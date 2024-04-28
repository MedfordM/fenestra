use std::collections::HashSet;
use std::fmt::Debug;

use windows::Win32::Graphics::Gdi::{DEVMODEA, HMONITOR, MONITORINFO};
use windows::Win32::UI::Shell::Common::DEVICE_SCALE_FACTOR;

use crate::data::common::direction::Direction;
use crate::data::group::Group;
use crate::data::window::Window;
use crate::data::workspace::Workspace;
use crate::win_api::monitor::{get_all, get_monitor, get_windows_on_monitor};

#[derive(Clone, Default)]
pub struct Monitor {
    pub hmonitor: HMONITOR,
    pub name: String,
    pub info: MONITORINFO,
    pub device_mode: DEVMODEA,
    pub scale: DEVICE_SCALE_FACTOR,
    pub workspaces: Vec<Workspace>,
    pub neighbors: Vec<(Direction, String)>,
    // pub focused: bool,
}

impl PartialEq for Monitor {
    fn eq(&self, other: &Self) -> bool {
        self.hmonitor == other.hmonitor || self.name == other.name
    }
}

impl Monitor {
    pub fn get_all_monitors() -> Vec<Monitor> {
        return get_all();
    }
    pub fn from(value: HMONITOR) -> Self {
        get_monitor(value)
    }

    pub fn current_workspace(&self) -> Workspace {
        return self
            .workspaces
            .iter()
            .find(|workspace| workspace.focused == true)
            .cloned()
            .expect("Unable to find current workspace");
    }

    pub fn get_workspace(&self, id: u32) -> Workspace {
        return self.workspaces[(id - 1) as usize].clone();
    }

    pub fn workspace_from_window(&self, window: &Window) -> Option<Workspace> {
        let search_result = self
            .workspaces
            .iter()
            .find(|workspace| workspace.all_windows().contains(window))
            .map(|w| w.to_owned());
        return search_result;
    }

    pub fn init_workspaces(hmonitor: HMONITOR) -> Vec<Workspace> {
        let mut workspaces: Vec<Workspace> = vec![];
        let default_workspace: Workspace = Workspace {
            id: 1,
            focused: true,
            groups: vec![Group {
                index: 0,
                children: Vec::new(),
                windows: get_windows_on_monitor(hmonitor),
            }],
        };
        workspaces.push(default_workspace);
        for i in 2..10 {
            let workspace: Workspace = Workspace {
                id: i,
                focused: false,
                groups: vec![Group {
                    index: 0,
                    children: Vec::new(),
                    windows: HashSet::new(),
                }],
            };
            workspaces.push(workspace);
        }
        return workspaces;
    }

    pub fn all_windows(&self) -> HashSet<Window> {
        let mut all_windows: HashSet<Window> = HashSet::new();
        self.workspaces.iter().for_each(|workspace| {
            all_windows.extend(workspace.all_windows().clone());
        });
        return all_windows;
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
