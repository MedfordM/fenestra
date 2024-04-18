use std::fmt::{Debug, Formatter};
use std::process::exit;
use log::error;
use crate::data::window::Window;
use crate::state::WORKSPACES;

#[derive(Clone, PartialEq)]
pub struct Workspace {
    pub id: u32,
    pub windows: Vec<Window>
}

impl Workspace {
    pub(crate) fn default() -> Self {
        Workspace {
            id: 1,
            windows: vec![]
        }
    }
    
    pub fn remove_window(&mut self, window: &Window) {
        let index = self.windows.iter().position(|w| w == window);
        if index.is_some() {
            self.windows.remove(index.unwrap());
        }
    }
    
    pub fn add_window(&mut self, window: &mut Window) {
        self.windows.push(window.clone());
    }
    
    pub fn find_workspace_by_window(window: &Window) -> &Workspace {
        let workspaces = WORKSPACES.lock().unwrap();
        let search_result = workspaces.iter().find(|workspace| workspace.windows.contains(window));
        if search_result.is_none() {
            error!("Unable to find workspace for window {}", window.title);
            exit(100);
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