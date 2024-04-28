use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

use crate::data::group::Group;
use log::debug;

use crate::data::window::Window;

#[derive(Clone)]
pub struct Workspace {
    pub id: u32,
    pub focused: bool,
    pub groups: Vec<Group>,
}

impl Workspace {
    pub fn all_windows(&self) -> HashSet<Window> {
        let mut windows = HashSet::new();
        self.groups.iter().for_each(|group| {
            windows.extend(group.all_windows());
        });
        return windows;
    }

    pub fn focus(&mut self) {
        debug!("Focusing workspace {}", self.id);
        let windows = self.all_windows();
        windows.iter().for_each(|window| {
            window.restore();
        });
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

    pub fn remove_window(&mut self, window: &Window) {
        let group_result = self.group_from_window(window);
        if group_result.is_none() {
            return;
        }
        let group = group_result.unwrap();
        let result = group.remove_window(window);
        if result {
            debug!("Removed '{}' from workspace {}", window.title, self.id);
            window.minimize();
        }
    }

    pub fn add_window(&mut self, window: &Window) {
        let groups = &mut self.groups;
        let mut target_group = groups.pop().unwrap();
        let result = target_group.add_window(window);
        if result {
            groups.push(target_group);
            debug!("Added '{}' to workspace {}", &window.title, &self.id);
        }
    }

    fn group_from_window(&mut self, window: &Window) -> Option<&mut Group> {
        return self
            .groups
            .iter_mut()
            .find(|group| group.windows.contains(window));
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
