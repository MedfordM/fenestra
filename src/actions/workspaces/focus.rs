use std::process::exit;
use std::str::FromStr;
use log::{debug, error};
use crate::data::action::Execute;
use crate::data::window::Window;
use crate::data::workspace::Workspace;
use crate::state::WORKSPACES;

#[derive(Clone, PartialEq)]
pub struct FocusWorkspace {
    pub id: u32
}

impl Execute for FocusWorkspace {
    fn execute(&self) {
        debug!("Focusing workspace {}", self.id);
        let mut workspaces = WORKSPACES.lock().unwrap();
        let workspace: Workspace;
        let result = workspaces.iter().position(|workspace| workspace.id == self.id);
        if result.is_none() {
            debug!("Creating workspace {}", self.id);
            workspace = Workspace{
                id: self.id,
                windows: vec![]
            };
            workspaces.push(workspace.clone());
        } else {
            let workspace_index= result.unwrap();
            workspace = workspaces.get(workspace_index).unwrap().clone();
        }
        let windows =  workspace.windows;
        // let window_titles: Vec<String> = windows.iter().map(|w| w.title.to_owned()).collect();
        // debug!("Windows on workspace {}: {:?}", self.id, &window_titles);
        let all_windows = Window::get_all_windows();
        let other_windows: Vec<Window> = Window::get_all_windows().iter().filter(|window| window.workspace_id != self.id).map(|window| window.clone()).collect();
        other_windows.iter().for_each(|window| {
            window.minimize();
        });
        windows.iter().for_each(|window| {
           window.restore();
        });
        
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
        Ok(FocusWorkspace{
            id: workspace_id.unwrap()
        })
    }
}

impl std::fmt::Debug for FocusWorkspace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Focus workspace {}", self.id)
    }
}
