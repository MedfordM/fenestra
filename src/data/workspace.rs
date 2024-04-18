use std::fmt::{Debug, Formatter};
use std::process::exit;

use log::{debug, error};

use crate::data::window::Window;
use crate::state::WORKSPACES;

#[derive(Clone, PartialEq)]
pub struct Workspace {
    pub id: u32,
    pub windows: Vec<Window>,
}

impl Workspace {
    pub(crate) fn default() -> Self {
        Workspace {
            id: 1,
            windows: Window::get_all_windows(),
        }
    }

    pub fn remove_window(&mut self, window: &Window) {
        let index = self.windows.iter().position(|w| w == window);
        if index.is_some() {
            debug!("Removed {} from workspace {}", window.title, self.id);
            self.windows.remove(index.unwrap());
            window.minimize();
        }
    }

    pub fn add_window(&mut self, window: &mut Window) {
        debug!("Added {} to workspace {}", window.title, self.id);
        self.windows.push(window.clone());
        window.restore();
    }

    pub fn find_workspace_by_id(id: u32) -> Box<Workspace> {
        debug!("Attempting to find workspace {}", id);
        let workspaces = WORKSPACES.lock().unwrap();
        let workspace: Box<Workspace>;
        let search_result = workspaces.iter().find(|workspace| workspace.id == id);
        if search_result.is_none() {
            debug!("Creating workspace {}", id);
            workspace = Box::new(Workspace {
                id,
                windows: vec![],
            });
        } else {
            workspace = Box::clone(search_result.unwrap());
        }
        debug!("Located workspace {}", id);
        return workspace;
    }

    pub fn find_workspace_by_window(window: &Window) -> Box<Workspace> {
        debug!("Attempting to find workspace containing {}", window.title);
        let workspaces = WORKSPACES.lock().unwrap();
        let target_workspace: Box<Workspace>;
        let search_result = workspaces
            .iter()
            .find(|workspace| workspace.windows.contains(window));
        if search_result.is_none() {
            error!(
                "Unable to find workspace for window {}, adding it to the default",
                window.title
            );
            target_workspace = Self::find_workspace_by_id(1);
        } else {
            target_workspace = Box::clone(search_result.unwrap());
        }
        debug!(
            "Located {} in workspace {}",
            window.title, &target_workspace.id
        );
        return target_workspace;
    }
}

impl Debug for Workspace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let windows = &self.windows;
        let window_titles: Vec<String> = windows.iter().map(|w| w.title.to_owned()).collect();
        write!(f, "Workspace {}: {:?}", &self.id, window_titles)
    }
}
