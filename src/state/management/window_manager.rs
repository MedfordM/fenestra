use crate::data::common::direction::{Direction, DirectionCandidate};
use crate::data::window::Window;
use crate::win_api;
use log::{debug, warn};
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::WS_MINIMIZE;

pub struct WindowManager {
    windows: Vec<Window>,
    active_window: HWND,
}

impl WindowManager {
    pub fn new(windows: Vec<Window>) -> Self {
        Self {
            windows,
            active_window: win_api::window::foreground_hwnd(),
        }
    }

    // pub fn current_window(&self) -> &HWND {
    //     &self.active_window
    // }
    //
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
            warn!("An attempt to add a window failed");
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
        self.windows.push(window);
        return true;
    }

    pub fn remove_window(&mut self, hwnd: &HWND) -> bool {
        if !self.managed_hwnds(false).contains(&hwnd) {
            return false;
        }
        // self.windows
        //     .iter()
        //     .find(|window| window.hwnd == *hwnd)
        //     .iter()
        //     .for_each(|window| debug!("Removing window {}", window.title));
        self.windows.retain(|w| w.hwnd != *hwnd);
        return true;
    }

    pub fn minimize(&mut self, hwnd: HWND) {
        let window = self.get_window(hwnd);
        let result = win_api::window::minimize(&window.hwnd);
        if result {
            debug!("Minimized '{}'", &window.title);
            self.remove_window(&hwnd);
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
            debug!("Restore '{}'", &window.title);
        } else {
            debug!("Unable to restore '{}'", &window.title);
        }
    }

    pub fn focus(&mut self, hwnd: HWND) {
        self.active_window = hwnd;
        let window = self.get_window(hwnd);
        let result = win_api::window::focus(&window.hwnd);
        if result {
            debug!("Focused '{}'", &window.title);
        } else {
            debug!("Unable to focus '{}'", &window.title);
        }
    }

    pub fn swap_dpi(&mut self, hwnd_1: HWND, hwnd_2: HWND) {
        let window_1_dpi = self
            .windows
            .iter()
            .find(|window| window.hwnd == hwnd_1)
            .map(|window| window.dpi)
            .expect("Unable to find window");
        let window_2_dpi = self
            .windows
            .iter()
            .find(|window| window.hwnd == hwnd_2)
            .map(|window| window.dpi)
            .expect("Unable to find window");
        {
            let window_1 = self.get_window(hwnd_1);
            window_1.dpi = window_2_dpi;
            let window_2 = self.get_window(hwnd_2);
            window_2.dpi = window_1_dpi;
        }
    }

    pub fn set_position(&mut self, hwnd: HWND, mut position: RECT, _offset: i32) {
        let window = self.get_window(hwnd);
        // let title_bar_height =
        //     win_api::window::get_window_coords(hwnd).rcClient.top - window.rect.top;
        // position.top += title_bar_height;
        let dpi = window.dpi;
        let current_dpi = win_api::window::get_dpi(hwnd);
        // let scale_factor = dpi as f32 / 96f32;
        // position.bottom = (position.bottom as f32 * scale_factor) as i32;
        // position.right = (position.right as f32 * scale_factor) as i32;
        window.rect = position;
        let shadow_width_left = window.shadow_rect.left - window.rect.left;
        let shadow_width_right = window.shadow_rect.right - window.rect.right;
        let shadow_width_bottom = window.shadow_rect.bottom - window.rect.bottom;
        position.left += shadow_width_left;
        position.right += shadow_width_right;
        position.bottom += shadow_width_bottom;
        // let mut adjusted_rect = adjust_for_dpi(&position, WINDOW_STYLE(window.style as u32), dpi);
        // adjusted_rect.bottom = (adjusted_rect.bottom as f32 * scale_factor) as i32;
        // adjusted_rect.right = (adjusted_rect.right as f32 * scale_factor) as i32;
        // window.rect = adjusted_rect;
        // let adj_left: i32 = window.rect.left + offset;
        // let width: i32 = window.rect.right - adj_left - offset;
        // let height: i32 = window.rect.bottom - window.rect.top - offset;
        // let rect = RECT {
        //     left: adj_left,
        //     top: position.top,
        //     right: width,
        //     bottom: height,
        // };
        win_api::window::set_position(&window.hwnd, window.rect, current_dpi != dpi);
        let result = win_api::window::restore(&window.hwnd);
        if result {
            debug!(
                "Set position for '{}': {{X: {}, Y: {}, width: {}, height: {}}}",
                window.title,
                window.rect.left,
                window.rect.top,
                window.rect.right,
                window.rect.bottom
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
        let all_window_titles: Vec<String> = self
            .windows
            .iter()
            .map(|window| String::from(&window.title))
            .collect();
        debug!("Currently managed windows: {:?}", all_window_titles);
        let window = self
            .windows
            .iter()
            .find(|window| window.hwnd == hwnd)
            .expect(
                &(String::from("Attempt to get an unmanaged window - ")
                    + &win_api::window::get_window_title(hwnd)),
            );
        let candidate_windows: Vec<&Window> = self
            .windows
            .iter()
            .filter(|window| candidate_hwnds.contains(&window.hwnd))
            .collect();
        // let candidate_window_titles: Vec<String> = candidate_windows
        //     .iter()
        //     .map(|window| String::from(&window.title))
        //     .collect();
        // debug!(
        //     "Finding closest window {} from {} with candidates {:?}",
        //     direction, &window.title, candidate_window_titles
        // );
        let origin = DirectionCandidate::from(&*window);
        let candidates = candidate_windows
            .iter()
            .map(|window| DirectionCandidate::from(*window))
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
        // let mut titles: Vec<String> = self
        //     .windows
        //     .iter()
        //     .map(|window| String::from(&window.title))
        //     .collect();
        // titles.sort();
        // debug!("Beginning window validation on windows {:?}", titles);
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
            if self.windows.iter().any(|w| w.hwnd == window.hwnd) {
                // Remove the old window state
                self.windows.retain(|w| w.hwnd != window.hwnd);
            } else {
                added_windows.push(window.hwnd);
            }
            self.windows.push(window);
        }
        // titles = self
        //     .windows
        //     .iter()
        //     .map(|window| String::from(&window.title))
        //     .collect();
        // titles.sort();
        // debug!("Completed window validation with windows {:?}", titles);
        return (removed_windows, added_windows);
    }

    fn get_window(&mut self, hwnd: HWND) -> &mut Window {
        self.windows
            .iter_mut()
            .find(|window| window.hwnd == hwnd)
            .expect("Unable to find the requested window")
    }
}
