use std::process::exit;
use std::str::FromStr;

use log::error;

use crate::data::action::Execute;
use crate::data::window::Window;
use crate::data::workspace::Workspace;
use crate::state::WORKSPACES;
use crate::win_api::window::get_foreground_window;

#[derive(Clone, PartialEq)]
pub struct MoveToWorkspace {
    pub id: u32,
}

impl Execute for MoveToWorkspace {
    fn execute(&self) {
        let current_window: Window = get_foreground_window();
        let mut workspaces = &mut WORKSPACES.lock().unwrap();
        let target_workspace = &mut Workspace::find_by_id(self.id, &mut workspaces);
        let current_workspace_result = Workspace::find_by_window(&current_window, &mut workspaces);
        if current_workspace_result.is_some() {
            let mut current_workspace = current_workspace_result.unwrap();
            current_workspace.remove_window(&current_window);
            workspaces[(current_workspace.id - 1) as usize] = current_workspace.clone();
        }
        target_workspace.add_window(&current_window);
        workspaces[(&self.id  - 1) as usize] = target_workspace.clone();
    }
}

impl FromStr for MoveToWorkspace {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input_up: String = input.to_ascii_uppercase();
        if !input_up.contains("SEND_TO_WORKSPACE_") {
            return Err(());
        }
        let workspace_id_str = input_up.strip_prefix("SEND_TO_WORKSPACE_").unwrap();
        let workspace_id = u32::from_str(workspace_id_str);
        if workspace_id.is_err() {
            error!("Unable to parse workspace id from {}", &workspace_id_str);
            exit(100);
        }
        Ok(MoveToWorkspace {
            id: workspace_id.unwrap(),
        })
    }
}

impl std::fmt::Debug for MoveToWorkspace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Move window to workspace {}", self.id)
    }
}
