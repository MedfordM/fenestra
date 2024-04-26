use std::{
    collections::HashSet,
    fmt::{Debug, Formatter},
};

use log::debug;

use crate::data::window::Window;

#[derive(Clone)]
pub struct Workspace {
    pub id: u32,
    pub focused: bool,
    pub windows: HashSet<Window>,
}

impl Workspace {
    pub fn focus(&mut self) {
        debug!("Focusing workspace {}", self.id);
        let windows = &self.windows;
        windows.iter().for_each(|window| {
            window.restore();
        });
        self.focused = true;
    }

    pub fn unfocus(&mut self) {
        debug!("Unfocusing workspace {}", self.id);
        let windows = &self.windows;
        windows.iter().for_each(|window| {
            window.minimize();
        });
        self.focused = false;
    }

    pub fn remove_window(&mut self, window: &Window) {
        let result = self.windows.remove(window);
        if result {
            debug!("Removed '{}' from workspace {}", window.title, self.id);
            window.minimize();
        }
    }

    pub fn add_window(&mut self, window: &Window) {
        let result = self.windows.insert(window.clone());
        if result {
            debug!("Added '{}' to workspace {}", &window.title, &self.id);
        }
    }
}

impl Debug for Workspace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let windows = &self.windows;
        let window_titles: Vec<String> = windows.iter().map(|w| w.title.to_owned()).collect();
        write!(f, "Workspace {}: {:?}", &self.id, window_titles)
    }
}

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
