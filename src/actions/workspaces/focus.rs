use std::process::exit;
use std::str::FromStr;
use std::sync::Mutex;

use log::{debug, error};

use crate::data::action::Execute;
use crate::state::MONITORS;
use crate::win_api::monitor::get_monitor_from_window;
use crate::win_api::window::get_foreground_handle;

#[derive(Clone, PartialEq)]
pub struct FocusWorkspace {
    pub id: u32,
}

impl Execute for FocusWorkspace {
    fn execute(&self) {
        let mut monitors = MONITORS.lock().unwrap();
        let window_handle = get_foreground_handle();
        let monitor_handle = get_monitor_from_window(window_handle);
        let monitor = monitors
            .iter_mut()
            .find(|monitor| monitor.hmonitor == monitor_handle)
            .expect("Unable to get current monitor");
        monitor.focus_workspace(self.id);
        // let current_index = (&current_workspace.id - 1) as usize;
        // let target_index = (&target_workspace.id - 1) as usize;
        // workspaces[current_index] = current_workspace.clone();
        // workspaces[target_index] = target_workspace.clone();
        // monitor.workspaces = workspaces;
        // let monitor_index = monitors
        //     .iter()
        //     .position(|mon| mon.hmonitor == monitor.hmonitor)
        //     .expect("Unable to find stateful index of monitor");
        // monitors[monitor_index] = monitor;
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
