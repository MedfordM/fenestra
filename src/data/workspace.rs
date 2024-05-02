use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

use crate::data::common::axis::Axis;
use crate::data::group::Group;
use log::debug;
use windows::Win32::Foundation::{HWND, RECT};

use crate::data::window::Window;
use crate::win_api::window::get_foreground_window;

#[derive(Clone)]
pub struct Workspace {
    pub id: u32,
    pub focused: bool,
    pub groups: Vec<Group>,
    pub split_axis: Axis,
    pub rect: RECT,
}

impl Workspace {
    pub fn all_windows(&self) -> HashSet<Window> {
        let mut windows = HashSet::new();
        self.groups.iter().for_each(|group| {
            windows.extend(group.get_windows().borrow().clone());
        });
        return windows;
    }

    /*
       Assign each group an even horizontal portion of the workspace, then let each group arrange
       their own windows how they see fit (horizontal, vertical, stacked)
    */
    pub fn arrange_windows(&mut self) {
        let all_windows = self.all_windows();
        if all_windows.len() == 1 {
            let window = all_windows.iter().next().unwrap();
            window.maximize();
            return;
        }
        let num_groups = self.groups.len();
        let rect_width = self.rect.right - self.rect.left;
        let group_width = rect_width as f32 / num_groups as f32;
        self.groups.iter_mut().for_each(|group| {
            let left = self.rect.left + (group_width as i32 * group.index as i32);
            let group_rect = RECT {
                left,
                right: left + group_width as i32,
                ..self.rect
            };
            group.arrange_windows(group_rect);
        });
    }

    pub fn contains_hwnd(&self, hwnd: &HWND) -> bool {
        self.all_windows().iter().any(|window| window.hwnd == *hwnd)
    }

    pub fn contains_window(&self, window: &Window) -> bool {
        self.all_windows().contains(window)
    }

    pub fn focus(&mut self) {
        debug!("Focusing workspace {}", self.id);
        let windows = self.all_windows();
        windows.iter().for_each(|window| {
            window.restore();
        });
        self.arrange_windows();
        self.focused = true;
    }

    pub fn unfocus(&mut self) {
        debug!("Unfocusing workspace {}", self.id);
        let windows = self.all_windows();
        windows.iter().for_each(|window| {
            window.minimize();
        });
        self.focused = false;
    }

    pub fn remove_hwnd(&mut self, hwnd: &HWND) -> bool {
        let group_result = self.group_from_hwnd(hwnd);
        if group_result.is_none() {
            return false;
        }
        let group = group_result.unwrap();
        let result = group.remove_hwnd(hwnd);
        if result {
            // debug!("Removed '{}' from workspace {}", window.title, self.id);
        }
        return result;
    }

    pub fn remove_window(&mut self, window: &Window) -> bool {
        let group_result = self.group_from_hwnd(&window.hwnd);
        if group_result.is_none() {
            return false;
        }
        let group = group_result.unwrap();
        let result = group.remove_window(window);
        if result {
            debug!("Removed '{}' from workspace {}", window.title, self.id);
        }
        return result;
    }

    pub fn add_window(&mut self, window: Window) -> bool {
        let title = String::from(&window.title);
        let result = self.groups[0].add_window(window);
        if result {
            debug!("Added '{}' to workspace {}", title, self.id);
        }
        return result;
    }

    fn group_from_hwnd(&mut self, hwnd: &HWND) -> Option<&mut Group> {
        return self
            .groups
            .iter_mut()
            .find(|group| group.contains_hwnd(hwnd));
    }

    pub fn current_group<'a, 'b>(&'a mut self) -> &'b mut Group
    where
        'a: 'b,
    {
        let current_window = get_foreground_window();
        return self
            .group_from_hwnd(&current_window.hwnd)
            .expect("Current window is not located on this workspace");
    }
}

impl Debug for Workspace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let windows = &self.all_windows();
        let window_titles: Vec<String> = windows.iter().map(|w| w.title.to_owned()).collect();
        write!(f, "Workspace {}: {:?}", &self.id, window_titles)
    }
}

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
