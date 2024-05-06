use std::str::FromStr;

use crate::actions::windows::focus::FocusWindow;
use crate::actions::windows::r#move::MoveWindow;
use crate::actions::workspaces::focus::FocusWorkspace;
use crate::actions::workspaces::r#move::MoveToWorkspace;
use crate::state::management::action_manager::ActionManager;

pub trait Action {
    fn execute(&self, action_manager: &mut ActionManager);
}

pub enum WindowManagerAction {
    FocusWindow(FocusWindow),
    MoveWindow(MoveWindow),
    FocusWorkspace(FocusWorkspace),
    MoveToWorkspace(MoveToWorkspace)
}

impl FromStr for WindowManagerAction {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let action: String = input.to_ascii_uppercase();
        if action.contains("FOCUS_WINDOW_") {
            return Ok(WindowManagerAction::FocusWindow(FocusWindow::from_str(action.as_str()).unwrap()));
        } else if action.contains("MOVE_WINDOW_") {
            return Ok(WindowManagerAction::MoveWindow(MoveWindow::from_str(action.as_str()).unwrap()));
        } else if action.contains("FOCUS_WORKSPACE_") {
            return Ok(WindowManagerAction::FocusWorkspace(FocusWorkspace::from_str(action.as_str()).unwrap()));
        } else if action.contains("SEND_TO_WORKSPACE_") {
            return Ok(WindowManagerAction::MoveToWorkspace(MoveToWorkspace::from_str(action.as_str()).unwrap()));
        }
        return Err(());
    }
}

