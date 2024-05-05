use windows::Win32::Graphics::Gdi::HMONITOR;
use crate::data::common::direction::{Direction, DirectionCandidate};
use crate::data::monitor::Monitor;
use crate::win_api;

pub struct MonitorManager {
    monitors: Vec<Monitor>
}

impl MonitorManager {
    pub fn new(monitors: Vec<Monitor>) -> Self {
        Self { monitors }
    }
    
    fn get_current_monitor(&mut self) -> &mut Monitor {
        let hwnd = win_api::window::foreground_hwnd();
        let hmonitor = win_api::monitor::hmonitor_from_hwnd(hwnd);
        return self.monitors
            .iter_mut()
            .find(|monitor| monitor.hmonitor == hmonitor)
            .expect("Unable to fetch monitor for requested hmonitor");
    }
    
    pub fn workspaces_for_monitor(&self, hmonitor: HMONITOR) -> &Vec<usize> {
        let monitor = self.monitors
            .iter()
            .find(|monitor| monitor.hmonitor == hmonitor)
            .expect("Unable to find monitor for requested hmonitor");
        return &monitor.workspaces;
    }
    
    pub fn find_nearest_in_direction(&mut self, direction: Direction) -> Option<HMONITOR> {
        let current_monitor = self.get_current_monitor();
        let origin = DirectionCandidate::from(&*current_monitor);
        let candidates = self.monitors.iter().map(|monitor| DirectionCandidate::from(monitor)).collect();
        let nearest_result = direction.find_nearest(&origin, candidates);
        return match nearest_result {
            Some(nearest) => Some(HMONITOR(nearest.id)),
            None => None
        };
    }
}