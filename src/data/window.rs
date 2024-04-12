use log::debug;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::WINDOWPLACEMENT;

use crate::data::common::direction::Direction;
use crate::data::monitor::Monitor;
use crate::win_api::window::{get_all, get_window, set_foreground_window};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Window {
    pub title: String,
    pub hwnd: HWND,
    pub thread_id: u32,
    pub process_id: u32,
    pub placement: WINDOWPLACEMENT,
    pub monitor: Monitor,
}

impl Window {
    pub fn get_all_windows() -> Vec<Window> {
        return get_all();
    }

    pub fn focus(&self) {
        set_foreground_window(self);
    }

    pub fn find_nearest_in_direction(&self, direction: &Direction) -> Window {
        let candidate_windows: Vec<Window> = Window::get_all_windows()
            .iter()
            .filter(|window| window != &self)
            .map(|window| window.clone())
            .collect();
        let monitor_windows: Vec<Window> = candidate_windows
            .iter()
            .filter(|window| window.monitor == self.monitor)
            .map(|window| window.clone())
            .collect();
        let monitor_rects: Vec<RECT> = monitor_windows
            .iter()
            .map(|window| window.placement.rcNormalPosition)
            .collect();
        let mut nearest_result: Option<(RECT, i32)> = direction.find_nearest(
            self.placement.rcNormalPosition,
            &monitor_rects,
            false,
            true,
            None,
        );
        if nearest_result.is_none() {
            // TODO: Check only the neighboring monitor in the requested direction
            let other_windows: Vec<Window> = candidate_windows
                .iter()
                .filter(|window| window.monitor != self.monitor)
                .map(|window| window.clone())
                .collect();
            let other_rects: Vec<RECT> = other_windows
                .iter()
                .map(|window| window.placement.rcNormalPosition)
                .collect();
            nearest_result = direction.find_nearest(
                self.placement.rcNormalPosition,
                &other_rects,
                false,
                true,
                None,
            );
            if nearest_result.is_none() {
                debug!("Unable to find nearest window");
            }
        }
        let (nearest_rect, nearest_delta): (RECT, i32) = nearest_result.unwrap();
        let nearest_window: Window = candidate_windows
            .iter()
            .find(|window| window.placement.rcNormalPosition == nearest_rect)
            .map(|window| window.clone())
            .unwrap();
        debug!(
            "Located {} nearest window: {} with delta {}",
            &direction, &nearest_window.title, nearest_delta
        );
        return nearest_window;
    }

    pub fn from(hwnd: HWND) -> Self {
        return get_window(hwnd);
    }
}
