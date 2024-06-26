use crate::data::window::Window;
use crate::win_api;
use log::{debug, warn};
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::{WS_MAXIMIZE, WS_MINIMIZE};

pub struct WindowManager {
    windows: Vec<Window>,
}

impl WindowManager {
    pub fn new(windows: Vec<Window>) -> Self {
        Self { windows }
    }

    pub fn managed_hwnds(&self, exclude_minimized: bool) -> Vec<HWND> {
        if exclude_minimized {
            return self
                .windows
                .iter()
                .filter(|window| window.style as u32 & WS_MINIMIZE.0 == 0)
                .map(|window| window.hwnd)
                .collect();
        }
        return self.windows.iter().map(|window| window.hwnd).collect();
    }

    pub fn add_window(&mut self, hwnd: HWND) -> bool {
        let window_result = win_api::window::get_window(hwnd);
        if window_result.is_none() {
            // warn!("An attempt to add a window failed");
            return false;
        }
        let window = window_result.unwrap();
        if self.windows.iter().any(|w| w.hwnd == window.hwnd) {
            // Remove the outdated window state
            let old_len = self.windows.len();
            self.windows.retain(|w| w.hwnd != window.hwnd);
            let new_len = self.windows.len();
            if old_len > new_len {
                // debug!("Removed old window state for '{}'", window.title);
            } else {
                warn!("Failed to remove old window state for '{}'", window.title);
                return false;
            }
        }
        // debug!("Adding window '{}'", window.title);
        let window_style = window.style;
        self.windows.push(window);
        return window_style & WS_MINIMIZE.0 as i32 == 0;
    }

    pub fn remove_window(&mut self, hwnd: HWND) -> bool {
        if !self.windows.iter().any(|window| window.hwnd == hwnd) {
            return false;
        }
        let old_len = self.windows.len();
        self.windows.retain(|w| w.hwnd != hwnd);
        let new_len = self.windows.len();
        return if old_len > new_len { true } else { false };
    }

    pub fn minimize(&mut self, hwnd: &HWND) {
        let window = self.get_window(&hwnd);
        let result = win_api::window::minimize(&window.hwnd);
        window.style = win_api::window::get_style(&window.hwnd);
        if result {
            debug!("Minimized '{}'", &window.title);
        } else {
            debug!("Unable to minimize '{}'", &window.title);
        }
    }

    pub fn maximize(&mut self, hwnd: &HWND) {
        let window = self.get_window(&hwnd);
        let result = win_api::window::maximize(&window.hwnd);
        window.style = win_api::window::get_style(&window.hwnd);
        if result {
            debug!("Maximized '{}'", &window.title);
        } else {
            debug!("Unable to maximize '{}'", &window.title);
        }
    }

    pub fn restore(&mut self, hwnd: &HWND) {
        let window = self.get_window(&hwnd);
        let result = win_api::window::restore(&window.hwnd);
        window.style = win_api::window::get_style(&window.hwnd);
        if result {
            debug!("Restore '{}'", &window.title);
        } else {
            debug!("Unable to restore '{}'", &window.title);
        }
    }

    pub fn focus(&mut self, hwnd: HWND) {
        let window = self.get_window(&hwnd);
        let result = win_api::window::focus(&window.hwnd);
        if result {
            debug!("Focused '{}'", &window.title);
        } else {
            debug!("Unable to focus '{}'", &window.title);
        }
    }

    pub fn close(&mut self, hwnd: HWND) {
        win_api::window::close(hwnd)
    }

    pub fn update_dpi(&mut self, hwnd: HWND) {
        self.get_window(&hwnd).dpi = 0;
    }

    pub fn set_positions(&mut self, positions: &Vec<(HWND, RECT)>) {
        for (hwnd, position) in positions {
            self.set_position(*hwnd, *position, 0);
        }
    }

    pub fn set_position(&mut self, hwnd: HWND, mut position: RECT, _offset: i32) {
        let window = self.get_window(&hwnd);
        if window.style & WS_MAXIMIZE.0 as i32 != 0 {
            // debug!("Skipping minimized window '{}'", window.title);
            win_api::window::restore(&window.hwnd);
        }
        window.rect = position;
        let current_dpi = win_api::window::get_dpi(hwnd);
        let delta_left = (window.info.rcWindow.left - window.info.rcClient.left).abs();
        let delta_right = (window.info.rcWindow.right - window.info.rcClient.right).abs();
        let delta_top = (window.info.rcWindow.top - window.info.rcClient.top).abs();
        let delta_bottom = (window.info.rcWindow.bottom - window.info.rcClient.bottom).abs();
        // Adjust for window frame
        position.left -= delta_left;
        position.right += delta_right;
        position.top -= delta_top;
        position.bottom += delta_bottom;
        // Adjust for frame border
        position.left -= window.border_thickness as i32;
        position.top -= window.border_thickness as i32;
        position.right += window.border_thickness as i32;
        position.bottom += window.border_thickness as i32;
        let dpi = window.dpi;
        win_api::window::set_position(&window.hwnd, position, current_dpi != dpi);
        debug!(
            "Set position for '{}': {{X: {}, Y: {}, width: {}, height: {}}}",
            window.title,
            window.rect.left,
            window.rect.top,
            window.rect.right - window.rect.left,
            window.rect.bottom - window.rect.top
        );
    }

    pub fn validate_windows(&mut self) -> (Vec<HWND>, Vec<HWND>) {
        // Remove any old windows
        let mut removed_windows = Vec::new();
        for i in 0..self.windows.len() {
            let hwnd = self.windows[i].hwnd;
            if win_api::window::get_window(hwnd).is_none() {
                self.windows.retain(|window| window.hwnd != hwnd);
                removed_windows.push(hwnd);
            }
        }
        // Add any new windows
        let mut added_windows = Vec::new();
        for window in win_api::window::get_all() {
            if self.windows.iter().any(|w| w.hwnd == window.hwnd) {
                // Remove the old window state
                self.windows.retain(|w| w.hwnd != window.hwnd);
            } else {
                added_windows.push(window.hwnd);
            }
            self.windows.push(window);
        }
        return (removed_windows, added_windows);
    }

    fn get_window(&mut self, hwnd: &HWND) -> &mut Window {
        self.windows
            .iter_mut()
            .find(|window| window.hwnd == *hwnd)
            .expect("Unable to find the requested window")
    }
}
