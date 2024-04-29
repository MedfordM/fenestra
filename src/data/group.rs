use crate::data::common::axis::Axis;
use crate::data::window::Window;
use std::collections::HashSet;
use windows::Win32::Foundation::RECT;

#[derive(Clone)]
pub struct Group {
    pub index: usize,
    pub windows: HashSet<Window>,
    pub split_axis: Axis,
}

impl Group {
    pub fn arrange_windows(&mut self, rect: RECT) {
        let mut windows = Vec::from_iter(self.windows.clone());
        let num_windows = windows.len();
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
        let mut index = 0;
        // debug!("Arranging group in rect {:?}", self.rect);
        for window in &mut windows {
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
            index = index + 1;
        }
        self.windows = HashSet::from_iter(windows);
    }
    pub fn contains_window(&self, window: &Window) -> bool {
        self.windows.contains(window)
    }

    pub fn remove_window(&mut self, window: &Window) -> bool {
        if !self.contains_window(window) {
            return false;
        }
        return self.windows.remove(window);
    }

    pub fn add_window(&mut self, window: &Window) -> bool {
        return self.windows.insert(window.clone());
    }
}
