use crate::data::common::direction::{Direction, DirectionCandidate};
use crate::data::window::Window;
use crate::win_api;
use log::{debug, warn};
use windows::Win32::Foundation::{HWND, RECT};

pub struct WindowManager {
    windows: Vec<Window>,
    active_window: HWND
}

impl WindowManager {
    pub fn new(windows: Vec<Window>) -> Self {
        Self { windows, active_window: win_api::window::foreground_hwnd() }
    }
    
    pub fn current_window(&self) -> &HWND {
        &self.active_window
    }

    pub fn add_window(&mut self, hwnd: HWND) -> bool {
        let window_result = win_api::window::get_window(hwnd);
        if window_result.is_none() {
            warn!("An attempt to add a window failed");
            return false;
        }
        let window = window_result.unwrap();
        if self.windows.contains(&window) {
            // Remove the outdated window state
            let old_len = self.windows.len();
            self.windows.retain(|w| w.hwnd != window.hwnd);
            let new_len = self.windows.len();
            if old_len > new_len {
                debug!("Removed old window state for {}", window.title);
            } else {
                warn!("Failed to remove old window state for {}", window.title);
                return false;
            }
        }
        debug!("Adding window {}", window.title);
        self.windows.push(window);
        return true;
    }

    pub fn minimize(&mut self, hwnd: HWND) {
        let window = self.get_window(hwnd);
        let result = win_api::window::minimize(&window.hwnd);
        if result {
            debug!("Minimized '{}'", &window.title);
        } else {
            debug!("Unable to minimize '{}'", &window.title);
        }
    }

    pub fn maximize(&mut self, hwnd: HWND) {
        let window = self.get_window(hwnd);
        let result = win_api::window::maximize(&window.hwnd);
        if result {
            debug!("Maximized '{}'", &window.title);
        } else {
            debug!("Unable to maximize '{}'", &window.title);
        }
    }

    pub fn restore(&mut self, hwnd: HWND) {
        let window = self.get_window(hwnd);
        let result = win_api::window::restore(&window.hwnd);
        if result {
            debug!("Maximized '{}'", &window.title);
        } else {
            debug!("Unable to maximize '{}'", &window.title);
        }
    }

    pub fn focus(&mut self, hwnd: HWND) {
        let window = self.get_window(hwnd);
        let result = win_api::window::focus(&window.hwnd);
        if result {
            debug!("Focused '{}'", &window.title);
        } else {
            debug!("Unable to focus '{}'", &window.title);
        }
    }

    pub fn set_position(&mut self, hwnd: HWND, position: RECT, offset: i32) {
        let window = self.get_window(hwnd);
        window.rect = position;
        let adj_left: i32 = window.rect.left + offset;
        let width: i32 = window.rect.right - adj_left - offset;
        let height: i32 = window.rect.bottom - window.rect.top - offset;
        win_api::window::set_position(&window.hwnd, position);
        let result = win_api::window::restore(&window.hwnd);
        if result {
            debug!(
                "Set position for '{}': {{X: {}, Y: {}, width: {}, height: {}}}",
                window.title, adj_left, window.rect.top, width, height
            );
        } else {
            debug!("Unable to set position for '{}'", &window.title);
        }
    }

    pub fn find_nearest_in_direction(
        &mut self,
        hwnd: HWND,
        direction: Direction,
        candidate_hwnds: Vec<HWND>,
    ) -> Option<HWND> {
        let window = self.get_window(hwnd);
        let origin = DirectionCandidate::from(&*window);
        let candidates = self
            .windows
            .iter()
            .filter(|window| candidate_hwnds.contains(&window.hwnd))
            .map(|window| DirectionCandidate::from(window))
            .collect();
        let nearest_result = direction.find_nearest(&origin, candidates);
        if nearest_result.is_some() {
            let nearest_window_hwnd = HWND(nearest_result.unwrap().id);
            return Some(nearest_window_hwnd);
        }
        debug!("Unable to find nearest window");
        return None;
    }

    pub fn validate_windows(&mut self) -> (Vec<HWND>, Vec<HWND>) {
        let mut titles: Vec<String> = self.windows.iter().map(|window| String::from(&window.title)).collect();
        titles.sort();
        debug!("Beginning window validation on windows {:?}", titles);
        let mut removed_windows = Vec::new();
        for i in 0..self.windows.len() {
            let hwnd = self.windows[i].hwnd;
            if win_api::window::get_window(hwnd).is_none() {
                self.windows.retain(|window| window.hwnd != hwnd);
                removed_windows.push(hwnd);
            }
        }
        let mut added_windows = Vec::new();
        for window in win_api::window::get_all() {
            if self.windows.contains(&window) {
                // Remove the old window state
                self.windows.retain(|w| w.hwnd != window.hwnd);
            } else {
                added_windows.push(window.hwnd);
            }
            self.windows.push(window);
        }
        titles = self.windows.iter().map(|window| String::from(&window.title)).collect();
        titles.sort();
        debug!("Completed window validation with windows {:?}", titles);
        return (removed_windows, added_windows);
    }

    fn get_window(&mut self, hwnd: HWND) -> &mut Window {
        self.windows
            .iter_mut()
            .find(|window| window.hwnd == hwnd)
            .expect("Unable to find the requested window")
    }
}
