use crate::data::common::direction::Direction;
use crate::data::monitor::Monitor;
use crate::win_api;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::HMONITOR;

pub struct MonitorManager {
    monitors: Vec<Monitor>,
}

impl MonitorManager {
    pub fn new(monitors: Vec<Monitor>) -> Self {
        Self { monitors }
    }

    pub fn monitor_from_hwnd(&self, hwnd: &HWND) -> HMONITOR {
        win_api::monitor::hmonitor_from_hwnd(*hwnd)
    }

    pub fn monitor_from_workspace(&self, workspace: usize) -> HMONITOR {
        self.monitors
            .iter()
            .find(|monitor| monitor.workspaces.contains(&workspace))
            .map(|monitor| monitor.hmonitor)
            .expect("No such workspace")
    }

    pub fn get_current(&self) -> HMONITOR {
        win_api::monitor::hmonitor_from_hwnd(win_api::window::foreground_hwnd())
    }

    pub fn neighbor_in_direction(
        &self,
        hmonitor: &HMONITOR,
        direction: &Direction,
    ) -> Option<HMONITOR> {
        let monitor = self
            .monitors
            .iter()
            .find(|monitor| monitor.hmonitor == *hmonitor)
            .expect("No such monitor");
        monitor.neighbors.get(direction).cloned()
    }

    pub fn workspaces_for_monitor(&self, hmonitor: HMONITOR) -> &Vec<usize> {
        let monitor = self
            .monitors
            .iter()
            .find(|monitor| monitor.hmonitor == hmonitor)
            .expect("Unable to find monitor for requested hmonitor");
        return &monitor.workspaces;
    }
}
