use log::debug;
use std::sync::Arc;

use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::{WINDOWINFO, WINDOWPLACEMENT};

use crate::data::common::direction::{Direction, DirectionCandidate, DirectionResult};
use crate::data::group::Group;
use crate::state::MONITORS;
use crate::win_api::monitor::get_monitor_from_window;
use crate::win_api::window::{
    get_window, maximize_window, minimize_window, restore_window, set_foreground_window,
    set_window_pos,
};

#[derive(Debug, Clone)]
pub struct Window {
    pub title: String,
    pub hwnd: HWND,
    pub thread_id: u32,
    pub process_id: u32,
    pub rect: RECT,
    pub bounding_rect: RECT,
    pub border_thickness: u32,
    pub info: WINDOWINFO,
    pub placement: WINDOWPLACEMENT,
    pub dpi: u32,
    pub style: i32,
    pub extended_style: i32,
    pub group: Arc<Group>,
}

impl Eq for Window {}

impl std::hash::Hash for Window {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.title.hash(state);
    }
}

impl Window {
    pub fn focus(&self) {
        set_foreground_window(self);
    }

    pub fn minimize(&self) {
        minimize_window(&self);
    }

    pub fn maximize(&self) {
        maximize_window(&self);
    }

    pub fn restore(&self) {
        restore_window(&self);
    }

    pub fn create_nearest_candidate(&self) -> DirectionCandidate<Window> {
        return DirectionCandidate {
            object: self.clone(),
            name: String::from(&self.title),
            rect: RECT {
                left: self.info.rcWindow.left + self.info.cxWindowBorders as i32,
                top: self.info.rcWindow.top + self.info.cyWindowBorders as i32,
                right: self.info.rcWindow.right,
                bottom: self.info.rcWindow.bottom,
            },
            offset_x: Some(self.info.cxWindowBorders),
            offset_y: Some(self.info.cyWindowBorders),
        };
    }

    pub fn find_nearest_in_direction(&mut self, direction: &Direction) -> Option<Window> {
        let mut nearest_result: Option<DirectionResult<Window>> = None;
        let origin = self.create_nearest_candidate();
        let mut all_candidates: Vec<Vec<DirectionCandidate<Window>>> = Vec::new();
        let current_hmonitor = get_monitor_from_window(self.hwnd);
        let current_monitor_cell = unsafe {
            MONITORS
                .iter()
                .map(|monitor| monitor.clone())
                .find(|monitor| monitor.borrow().hmonitor == current_hmonitor)
                .expect("Unable to find current monitor")
                .clone()
        };
        let mut current_monitor = current_monitor_cell.borrow_mut();
        let current_workspace = current_monitor.current_workspace();
        let workspace_windows = current_workspace.all_windows();
        all_candidates.push(
            workspace_windows
                .iter()
                .filter(|window| !window.eq(&self))
                .map(|window| window.create_nearest_candidate())
                .collect(),
        );
        let current_group = current_workspace.current_group();
        let group_windows = &current_group.get_windows();
        all_candidates.push(
            group_windows
                .borrow()
                .iter()
                .filter(|window| !window.eq(&self))
                .map(|window| window.create_nearest_candidate())
                .collect(),
        );

        let neighbors = &current_monitor.neighbors;
        let neighbor_monitor_result = neighbors.get(direction);
        if neighbor_monitor_result.is_some() {
            debug!(
                "Searching neighboring {} monitor for nearest window",
                direction
            );
            let neighbor_monitor_result = neighbor_monitor_result.unwrap();
            let neighbor_monitor_cell = neighbor_monitor_result.clone();
            let mut neighbor_monitor = neighbor_monitor_cell.borrow_mut();
            let neighbor_monitor_workspace = neighbor_monitor.current_workspace();
            let neighbor_monitor_windows = neighbor_monitor_workspace.all_windows();
            all_candidates.push(
                neighbor_monitor_windows
                    .iter()
                    .map(|window| window.create_nearest_candidate())
                    .collect(),
            );
        }
        for candidate_set in all_candidates {
            nearest_result = direction.find_nearest(&origin, candidate_set);
            if nearest_result.is_some() {
                let nearest_window = nearest_result.unwrap().object;
                debug!("Found nearest window '{}'", nearest_window.title);
                return Some(nearest_window.clone());
            }
        }
        debug!("Unable to find nearest window");
        return None;
    }

    pub fn move_in_direction(&mut self, direction: &Direction) {
        let nearest_result = self.find_nearest_in_direction(direction);
        if nearest_result.is_some() {
            let target_window = nearest_result.unwrap();
            self.swap_windows(target_window);
        } else {
            debug!("Moving window {} {}", String::from(&self.title), direction);
        }
    }

    fn swap_windows(&mut self, mut window: Window) {
        debug!(
            "Swapping window {} with {}",
            String::from(&self.title),
            String::from(&window.title)
        );
        let current_pos: RECT = self.rect;
        let target_pos: RECT = window.rect;
        // Calculate drop shadow width
        // let current_delta = 0;
        // let target_delta = 0;
        let current_delta = self.bounding_rect.left - self.rect.left;
        let target_delta = window.bounding_rect.left - window.rect.left;
        // let current_delta = self.bounding_rect.top - self.rect.top;
        // let target_delta = window.bounding_rect.top - window.rect.top;
        window.set_position(current_pos, Some(current_delta - target_delta));
        self.set_position(target_pos, Some(target_delta - current_delta));
    }

    fn swap_window_positions(&mut self, mut window: Window) {
        debug!(
            "Swapping window positions of {} and {}",
            String::from(&self.title),
            String::from(&window.title)
        );
        let current_pos: RECT = self.rect;
        let target_pos: RECT = window.rect;
        // Calculate drop shadow width
        // let current_delta = 0;
        // let target_delta = 0;
        let current_delta = self.bounding_rect.left - self.rect.left;
        let target_delta = window.bounding_rect.left - window.rect.left;
        // let current_delta = self.bounding_rect.top - self.rect.top;
        // let target_delta = window.bounding_rect.top - window.rect.top;
        window.set_position(current_pos, Some(current_delta - target_delta));
        self.set_position(target_pos, Some(target_delta - current_delta));
    }

    pub fn from(hwnd: HWND) -> Option<Self> {
        return get_window(hwnd);
    }

    pub fn set_position(&mut self, position: RECT, offset: Option<i32>) {
        //debug!("Old window position for {}: {:?} with offset {}", self.title, &self.rect, offset);
        self.rect = position;
        set_window_pos(self, offset.unwrap_or(0));
    }
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd
            || self.title == other.title
            || self.thread_id == other.thread_id
            || self.process_id == other.process_id
    }
}
