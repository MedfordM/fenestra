use crate::data::common::axis::Axis;
use crate::data::window::Window;
use log::debug;
use std::cell::RefCell;
use windows::Win32::Foundation::{HWND, RECT};

#[derive(Clone)]
pub struct Group {
    pub index: usize,
    windows: RefCell<Vec<Window>>,
    pub split_axis: Axis,
}

impl Group {
    pub fn new(index: usize, windows: Vec<Window>, split_axis: Axis) -> Self {
        Self {
            index,
            windows: RefCell::new(windows),
            split_axis,
        }
    }

    pub fn get_windows(&self) -> &RefCell<Vec<Window>> {
        &self.windows
    }

    pub fn swap_windows(&mut self, window_1: &mut Window, window_2: &mut Window) {
        let index_1 = self
            .windows
            .borrow()
            .iter()
            .position(|w| w == window_1)
            .unwrap();
        let index_2 = self
            .windows
            .borrow()
            .iter()
            .position(|w| w == window_2)
            .unwrap();
        self.windows.borrow_mut().swap(index_1, index_2);
    }

    pub fn arrange_windows(&mut self, rect: RECT) {
        let mut windows = self.windows.borrow().clone();
        let window_titles: Vec<String> = windows
            .iter()
            .map(|window| String::from(window.title.clone()))
            .collect();
        let num_windows = windows.len();
        debug!("Arranging {} windows: {:?}", num_windows, window_titles);
        let rect_width = rect.right - rect.left;
        let rect_height = rect.bottom - rect.top;
        let section_width = match self.split_axis {
            Axis::HORIZONTAL => rect_width as f32,
            Axis::VERTICAL => rect_width as f32 / num_windows as f32,
        };
        let section_height = match self.split_axis {
            Axis::HORIZONTAL => rect_height as f32 / num_windows as f32,
            Axis::VERTICAL => rect_height as f32,
        };
        // debug!(
        //     "Computed section width as {} with {} windows",
        //     section_width, num_windows
        // );
        // debug!("Arranging group in rect {:?}", self.rect);
        let mut index = 0;
        for window in windows.iter_mut() {
            let new_position = match self.split_axis {
                Axis::HORIZONTAL => {
                    let top = rect.top + (section_height as i32 * index);
                    RECT {
                        top,
                        bottom: top + section_height as i32,
                        ..rect
                    }
                }
                Axis::VERTICAL => {
                    let left = rect.left + (section_width as i32 * index);
                    RECT {
                        left,
                        right: left + section_width as i32,
                        ..rect
                    }
                }
            };
            // debug!("Arranging '{}' to {:?}", window.title, new_position);
            window.restore();
            window.set_position(new_position, None);
            index += 1;
        }
        *self.windows.borrow_mut() = windows;
    }

    pub fn contains_hwnd(&self, hwnd: &HWND) -> bool {
        self.windows
            .borrow()
            .iter()
            .any(|window| window.hwnd == *hwnd)
    }

    pub fn remove_hwnd(&mut self, hwnd: &HWND) -> bool {
        if !self.contains_hwnd(hwnd) {
            return false;
        }
        let mut windows = self.windows.borrow_mut();
        let old_len = windows.len();
        windows.retain(|window| window.hwnd != *hwnd);
        let new_len = windows.len();
        return new_len < old_len;
    }

    pub fn contains_window(&self, window: &Window) -> bool {
        self.windows.borrow().contains(window)
    }

    pub fn remove_window(&mut self, window: &Window) -> bool {
        let mut windows = self.windows.borrow_mut();
        let old_len = windows.len();
        windows.retain(|w| w != window);
        let new_len = windows.len();
        return old_len > new_len;
    }

    pub fn add_window(&mut self, window: Window) -> bool {
        let mut windows = self.windows.borrow_mut();
        if windows.contains(&window) {
            let index = windows.iter().position(|w| w == &window).unwrap();
            windows[index] = window;
        } else {
            windows.push(window);
        }
        return true;
    }
}
