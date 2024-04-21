use std::process::exit;
use std::str::FromStr;

use log::error;

use crate::data::action::Execute;
use crate::data::workspace::Workspace;
use crate::state::WORKSPACES;

#[derive(Clone, PartialEq)]
pub struct FocusWorkspace {
    pub id: u32,
}

impl Execute for FocusWorkspace {
    fn execute(&self) {
        let mut workspaces = WORKSPACES.lock().unwrap();
        let mut current_workspace = Workspace::current(&workspaces);
        current_workspace.unfocus();
        let mut target_workspace: Box<Workspace> = Workspace::find_by_id(self.id, &mut workspaces);
        target_workspace.focus();
        workspaces[(&self.id  - 1) as usize] = target_workspace.clone();
        workspaces[(current_workspace.id - 1) as usize] = current_workspace.clone();
    }
}

impl FromStr for FocusWorkspace {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input_up: String = input.to_ascii_uppercase();
        if !input_up.contains("FOCUS_WORKSPACE_") {
            return Err(());
        }
        let workspace_id_str = input_up.strip_prefix("FOCUS_WORKSPACE_").unwrap();
        let workspace_id = u32::from_str(workspace_id_str);
        if workspace_id.is_err() {
            error!("Unable to parse workspace id from {}", &workspace_id_str);
            exit(100);
        }
        Ok(FocusWorkspace {
            id: workspace_id.unwrap(),
        })
    }
}

impl std::fmt::Debug for FocusWorkspace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Focus workspace {}", self.id)
    }
}
