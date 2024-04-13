use std::fmt::Debug;

use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Gdi::HMONITOR;

use crate::data::common::direction::Direction;
use crate::data::workspace::Workspace;
use crate::win_api::monitor::{get_all, get_monitor};

#[derive(Clone, PartialEq, Default)]
pub struct Monitor {
    pub hmonitor: HMONITOR,
    pub name: String,
    pub position: RECT,
    pub workspaces: Vec<Workspace>,
    pub neighbors: Vec<(Direction, String)>,
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
            self.name, self.position, self.workspaces, self.neighbors
        )
    }
}
