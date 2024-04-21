use std::fmt::{Debug, Formatter};

use log::{debug, error};

use crate::data::window::Window;

#[derive(Clone, PartialEq)]
pub struct Workspace {
    pub id: u32,
    pub windows: Vec<Window>,
}

impl Workspace {
    pub fn focus(&self) {
        let windows = &self.windows;
        let window_titles: Vec<String> = windows.iter().map(|w| w.title.to_owned()).collect();
        debug!("Windows on workspace {}: {:?}", self.id, &window_titles);
        let other_windows: Vec<Window> = Window::get_all_windows()
            .iter()
            .filter(|window| !windows.contains(window))
            .map(|window| window.clone())
            .collect();
        let other_window_titles: Vec<String> = other_windows.iter().map(|w| w.title.to_owned()).collect();
        debug!("Other windows: {:?}", other_window_titles);
        other_windows.iter().for_each(|window| {
            window.minimize();
        });
        windows.iter().for_each(|window| {
            window.restore();
        });
    }

    pub fn remove_window(&mut self, window: &Window) {
        let index = self.windows.iter().position(|w| w == window);
        if index.is_some() {
            debug!("Removed {} from workspace {}", window.title, self.id);
            self.windows.remove(index.unwrap());
            window.minimize();
        }
    }

    pub fn add_window(&mut self, window: &Window) {
        debug!("Added {} to workspace {}", &window.title, &self.id);
        self.windows.push(window.clone());
    }

    pub fn find_by_id(id: u32, workspaces: &mut Vec<Box<Workspace>>) -> Box<Workspace> {
        debug!("Attempting to find workspace {}", id);
        let search_result = workspaces.iter().find(|workspace| workspace.id == id).map(|w| w.to_owned());
        if search_result.is_none() {
            debug!("Creating workspace {}", id);
            let workspace: Box<Workspace> = Box::new(Workspace {
                id,
                windows: vec![],
            });
            workspaces.push(Box::clone(&workspace));
            return workspace;
        }
        return search_result.unwrap();
    }

    pub fn find_by_window(window: &Window, workspaces: &mut Vec<Box<Workspace>>) -> Box<Workspace> {
        debug!("Attempting to find workspace containing {}", window.title);
        let search_result = workspaces
            .iter()
            .find(|workspace| workspace.windows.contains(window))
            .map(|w| w.to_owned());
        if search_result.is_none() {
            error!(
                "Unable to find workspace for window {}, adding it to the default",
                window.title
            );
            let mut default_workspace: Box<Workspace> = Self::find_by_id(1, workspaces);
            default_workspace.add_window(window);
            return default_workspace;
        }
        return search_result.unwrap();
    }
}

impl Debug for Workspace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let windows = &self.windows;
        let window_titles: Vec<String> = windows.iter().map(|w| w.title.to_owned()).collect();
        write!(f, "Workspace {}: {:?}", &self.id, window_titles)
    }
}
