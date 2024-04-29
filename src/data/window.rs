use std::collections::HashSet;

use log::debug;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::{SW_SHOWMINIMIZED, WINDOWINFO, WINDOWPLACEMENT};

use crate::data::common::direction::Direction;
use crate::data::monitor::Monitor;
use crate::win_api::monitor::get_monitor_from_window;
use crate::win_api::window::{
    get_all, get_window, maximize_window, minimize_window, restore_window, set_foreground_window,
    set_window_pos,
};

#[derive(Debug, Clone, Default)]
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
}

impl Eq for Window {}

impl std::hash::Hash for Window {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.title.hash(state);
    }
}

impl Window {
    pub fn get_all_windows() -> HashSet<Window> {
        return get_all();
    }

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

    pub fn find_nearest_in_direction(&self, direction: &Direction) -> Window {
        let candidate_windows: Vec<Window> = Window::get_all_windows()
            .iter()
            .filter(|window| {
                window != &self && window.placement.showCmd != SW_SHOWMINIMIZED.0 as u32
            })
            .map(|window| Window {
                info: WINDOWINFO {
                    rcWindow: RECT {
                        left: window.info.rcWindow.left + window.info.cxWindowBorders as i32,
                        top: window.info.rcWindow.top + window.info.cyWindowBorders as i32,
                        right: window.info.rcWindow.right,
                        bottom: window.info.rcWindow.bottom,
                    },
                    ..window.info
                },
                ..window.clone()
            })
            .collect();
        let hmonitor = get_monitor_from_window(self.hwnd);
        let mut monitor = Monitor::from(hmonitor);
        let workspace = monitor.current_workspace();
        let workspace_windows: HashSet<Window> = candidate_windows
            .iter()
            .filter(|window| workspace.all_windows().contains(window))
            .map(|window| window.clone())
            .collect();
        let workspace_rects: Vec<(String, RECT, Option<u32>, Option<u32>)> = workspace_windows
            .iter()
            .map(|window| {
                (
                    String::from(&window.title),
                    window.info.rcWindow,
                    Some(window.info.cxWindowBorders),
                    Some(window.info.cyWindowBorders),
                )
            })
            .collect();
        let mut nearest_result: Option<(String, i32)> = direction.find_nearest(
            (
                String::from(&self.title),
                RECT {
                    // TODO: Is this necessary; isn't this done above?
                    left: self.info.rcWindow.left + self.info.cxWindowBorders as i32,
                    top: self.info.rcWindow.top + self.info.cyWindowBorders as i32,
                    right: 0,
                    bottom: 0,
                },
                Some(self.info.cxWindowBorders),
                Some(self.info.cyWindowBorders),
            ),
            &workspace_rects,
        );
        if nearest_result.is_none() {
            // TODO: Check only the neighboring monitor in the requested direction
            let other_windows: Vec<Window> = candidate_windows
                .iter()
                .filter(|window| !monitor.all_windows().contains(window))
                .map(|window| window.clone())
                .collect();
            let other_rects: Vec<(String, RECT, Option<u32>, Option<u32>)> = other_windows
                .iter()
                .map(|window| {
                    (
                        String::from(&window.title),
                        window.info.rcWindow,
                        None,
                        None,
                    )
                })
                .collect();
            nearest_result = direction.find_nearest(
                (
                    self.title.clone(),
                    RECT {
                        left: self.info.rcWindow.left + self.info.cxWindowBorders as i32,
                        top: self.info.rcWindow.top + self.info.cyWindowBorders as i32,
                        right: 0,
                        bottom: 0,
                    },
                    Some(self.info.cxWindowBorders),
                    Some(self.info.cyWindowBorders),
                ),
                &other_rects,
            );
            if nearest_result.is_none() {
                debug!("Unable to find window {} from {}", &direction, self.title);
                return self.clone();
            }
        }
        let (nearest_window, nearest_delta): (String, i32) = nearest_result.unwrap();
        let nearest_window: Window = candidate_windows
            .iter()
            .find(|window| window.title == nearest_window)
            .map(|window| window.clone())
            .unwrap();
        debug!(
            "Located nearest window({}): {} at a distance of {}",
            &direction, &nearest_window.title, nearest_delta
        );
        return nearest_window;
    }

    pub fn swap_windows(&mut self, mut window: Window) {
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

    // fn set_placement(&mut self, placement: &WINDOWPLACEMENT) {
    //     set_window_placement(self, placement);
    //     self.placement = placement.clone();
    // }
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd
            || self.title == other.title
            || self.thread_id == other.thread_id
            || self.process_id == other.process_id
    }
}
