use std::{collections::HashSet, fmt::{Debug, Formatter}};

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
        let windows = &self.windows;
        let window_titles: Vec<String> = windows.iter().map(|w| w.title.to_owned()).collect();
        debug!("Focusing workspace {} windows: {:?}", self.id, &window_titles);
        windows.iter().for_each(|window| {
            window.restore();
        });
        self.focused = true;
    }

    pub fn unfocus(&mut self) {
        let windows = &self.windows;
        let window_titles: Vec<String> = windows.iter().map(|w| w.title.to_owned()).collect();
        debug!("Unfocusing workspace {} windows: {:?}", self.id, &window_titles);
        windows.iter().for_each(|window| {
            window.minimize();
        });
        self.focused = false;
    }

    pub fn remove_window(&mut self, window: &Window) {
        let result = self.windows.remove(window);
        if result {
            debug!("Removed {} from workspace {}", window.title, self.id);
            window.minimize();
        }
    }

    pub fn add_window(&mut self, window: &Window) {
        let result = self.windows.insert(window.clone());
        if result {
            debug!("Added {} to workspace {}", &window.title, &self.id);
        }
    }

    pub fn current(workspaces: &Vec<Box<Workspace>>) -> Box<Workspace> {
        let result = workspaces.iter().find(|workspace| workspace.focused == true).cloned().expect("No currently focused workspace");
        return result;
    }

    pub fn find_by_id(id: u32, workspaces: &mut Vec<Box<Workspace>>) -> Box<Workspace> {
        return workspaces[(id - 1) as usize].clone();
    }

    pub fn find_by_window(window: &Window, workspaces: &mut Vec<Box<Workspace>>) -> Option<Box<Workspace>> {
        let search_result = workspaces
            .iter()
            .find(|workspace| workspace.windows.contains(window))
            .map(|w| w.to_owned());
        return search_result;
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