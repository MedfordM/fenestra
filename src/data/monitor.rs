use std::fmt::Debug;

use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Gdi::HMONITOR;

use crate::data::workspace::Workspace;
use crate::win_api::monitor::{get_all, get_monitor};

#[derive(Clone, PartialEq, Default)]
pub struct Monitor {
    pub name: String,
    pub position: RECT,
    pub workspaces: Vec<Workspace>,
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
            "Monitor {}: {:?}, {:?}",
            self.name, self.position, self.workspaces
        )
    }
}
