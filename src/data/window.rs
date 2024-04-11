use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::WINDOWPLACEMENT;

use crate::win_api::window::{get_all_windows, set_foreground_window};

#[derive(Debug, Clone, PartialEq)]
pub struct Window {
    pub title: String,
    pub hwnd: HWND,
    pub thread_id: u32,
    pub process_id: u32,
    pub placement: WINDOWPLACEMENT,
}

impl Window {
    pub fn focus(&self) {
        set_foreground_window(self);
    }

    pub fn find_nearest_in_direction(&self, direction: &String) -> Window {
        let mut nearest_window: (Window, i32) = (self.clone(), i32::MAX);
        let all_windows: Vec<Window> = get_all_windows();
        all_windows.iter().for_each(|candidate_window| {
            // Skip evaluation if candidate window is in the same place as the active one
            if candidate_window.placement.rcNormalPosition == self.placement.rcNormalPosition {
                return;
            }
            let active: i32; // focused window
            let candidate: i32; // window currently being evaluated
            match direction.to_ascii_uppercase().as_str() {
                "LEFT" => {
                    active = self.placement.rcNormalPosition.left;
                    candidate = candidate_window.placement.rcNormalPosition.right;
                }
                "RIGHT" => {
                    active = self.placement.rcNormalPosition.right;
                    candidate = candidate_window.placement.rcNormalPosition.left;
                }
                "UP" => {
                    active = self.placement.rcNormalPosition.top;
                    candidate = candidate_window.placement.rcNormalPosition.bottom;
                }
                "DOWN" => {
                    active = self.placement.rcNormalPosition.bottom;
                    candidate = candidate_window.placement.rcNormalPosition.top;
                }
                _ => return,
            }
            let delta: i32 = (candidate - active).abs();
            if delta < nearest_window.1 {
                nearest_window = (candidate_window.clone(), delta);
                return;
            }
        });
        return nearest_window.0;
    }
}
