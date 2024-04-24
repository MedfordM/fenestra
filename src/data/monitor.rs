use std::fmt::Debug;

use windows::Win32::Graphics::Gdi::{DEVMODEA, DISPLAY_DEVICEA, HMONITOR, MONITORINFO};

use crate::data::common::direction::Direction;
use crate::data::workspace::Workspace;
use crate::win_api::monitor::{get_all, get_monitor};

#[derive(Clone, Default)]
pub struct Monitor {
    pub hmonitor: HMONITOR,
    pub name: String,
    pub info: MONITORINFO,
    // pub display: DISPLAY_DEVICEA,
    pub device_mode: DEVMODEA,
    pub workspaces: Vec<Workspace>,
    pub neighbors: Vec<(Direction, String)>,
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
