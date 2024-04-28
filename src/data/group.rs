use crate::data::window::Window;
use std::collections::HashSet;
use std::ops::Deref;

#[derive(Clone)]
pub struct Group {
    pub index: usize,
    pub children: Vec<Group>,
    pub windows: HashSet<Window>,
}

impl Group {
    pub fn all_windows(&self) -> HashSet<Window> {
        let mut windows = self.windows.clone();
        self.children.iter().for_each(|child| {
            windows.extend(child.windows.clone());
        });
        return windows;
    }

    pub fn contains_window(&self, window: &Window) -> bool {
        if self.all_windows().contains(window) {
            return true;
        }
        return false;
    }

    pub fn remove_window(&mut self, window: &Window) -> bool {
        if !self.contains_window(window) {
            return false;
        }
        let target_group = self.group_from_window(window).expect("unable");
        let target_windows = &mut target_group.windows;
        return target_windows.remove(window);
    }

    pub fn add_window(&mut self, window: &Window) -> bool {
        return self.windows.insert(window.clone());
    }

    pub fn group_from_window(&mut self, window: &Window) -> Option<&mut Group> {
        if !self.contains_window(window) {
            return None;
        }
        if self.windows.contains(window) {
            return Some(self);
        }
        for child in &mut self.children {
            let result = child.group_from_window(window);
            if result.is_some() {
                return result;
            }
        }
        return None;
    }
}
