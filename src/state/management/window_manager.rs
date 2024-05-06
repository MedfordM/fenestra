use log::debug;
use windows::Win32::Foundation::{HWND, RECT};
use crate::data::common::direction::{Direction, DirectionCandidate};
use crate::data::window::Window;
use crate::win_api;

pub struct WindowManager {
    windows: Vec<Window>
}

impl WindowManager {
    pub fn new(windows: Vec<Window>) -> Self {
        Self { windows }
    }
    
    pub fn minimize(&mut self, hwnd: HWND) {
        let window= self.get_window(hwnd);
        let result = win_api::window::minimize(&window.hwnd);
        if result {
            debug!("Minimized '{}'",&window.title);
        } else {
            debug!("Unable to minimize '{}'",&window.title);
        }
    }
    
    pub fn maximize(&mut self, hwnd: HWND) {
        let window= self.get_window(hwnd);
        let result = win_api::window::maximize(&window.hwnd);
        if result {
            debug!("Maximized '{}'",&window.title);
        } else {
            debug!("Unable to maximize '{}'",&window.title);
        }
    }

    pub fn restore(&mut self, hwnd: HWND) {
        let window= self.get_window(hwnd);
        let result = win_api::window::restore(&window.hwnd);
        if result {
            debug!("Maximized '{}'",&window.title);
        } else {
            debug!("Unable to maximize '{}'",&window.title);
        }
    }
    
    pub fn focus(&mut self, hwnd: HWND) {
        let window= self.get_window(hwnd);
        let result = win_api::window::focus(&window.hwnd);
        if result {
            debug!("Focused '{}'",&window.title);
        } else {
            debug!("Unable to focus '{}'",&window.title);
        }
    }
    
    pub fn set_position(&mut self, hwnd: HWND, position: RECT, offset: i32) {
        let window= self.get_window(hwnd);
        window.rect = position;
        let adj_left: i32 = window.rect.left + offset;
        let width: i32 = window.rect.right - adj_left - offset;
        let height: i32 = window.rect.bottom - window.rect.top - offset;
        let result = win_api::window::restore(&window.hwnd);
        if result {
            debug!(
                "Set for '{}': {{X: {}, Y: {}, width: {}, height: {}}}",
                window.title, adj_left, window.rect.top, width, height
            );
        } else {
            debug!("Unable to maximize '{}'",&window.title);
        }
    }
    
    pub fn find_nearest_in_direction(&mut self, hwnd: HWND, direction: Direction) -> Option<HWND> {
        let window = self.get_window(hwnd);
        let origin = DirectionCandidate::from(&*window);
        let candidates = self.windows.iter().map(|window| DirectionCandidate::from(window)).collect();
        let nearest_result = direction.find_nearest(&origin, candidates);
        if nearest_result.is_some() {
            let nearest_window_hwnd = HWND(nearest_result.unwrap().id);
            return Some(nearest_window_hwnd);
        }
        debug!("Unable to find nearest window");
        return None;
    }
    
    pub fn validate_windows(&mut self) -> (Vec<HWND>, Vec<HWND>) {
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
                // Update the window state
                self.windows.retain(|w| w != &window);
                self.windows.push(window);
            } else {
                added_windows.push(window.hwnd);
            }
        }
        return (removed_windows, added_windows);
    }
    
    fn get_window(&mut self, hwnd: HWND) -> &mut Window {
        self.windows
            .iter_mut()
            .find(|window| window.hwnd == hwnd)
            .expect("Unable to find the requested window")
    }

}
