use std::fmt::Debug;

use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Gdi::HMONITOR;

use crate::data::common::direction::Direction;
use crate::data::workspace::Workspace;
use crate::win_api::monitor::{get_all, get_monitor};

#[derive(Clone, PartialEq, Default)]
pub struct Monitor {
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

    pub fn contains_rect(&self, window_pos: RECT) -> bool {
        let monitor_pos = &self.position;
        return monitor_pos.left < window_pos.left
            && monitor_pos.top < window_pos.top
            && monitor_pos.right > window_pos.right
            && monitor_pos.bottom > window_pos.bottom;
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
