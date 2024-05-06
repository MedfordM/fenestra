use std::process::exit;
use std::str::FromStr;

use crate::data::action::Action;
use crate::state::management::state_manager::StateManager;
use log::error;

pub struct MoveToWorkspace {
    pub id: usize,
}

impl Action for MoveToWorkspace {
    fn execute(&self, state_manager: &mut StateManager) {
        state_manager.move_to_workspace(self.id)
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
        let workspace_id = usize::from_str(workspace_id_str);
        if workspace_id.is_err() {
            error!("Unable to parse workspace id from {}", &workspace_id_str);
            exit(100);
        }
        Ok(MoveToWorkspace {
            id: workspace_id.unwrap(),
        })
    }
}
