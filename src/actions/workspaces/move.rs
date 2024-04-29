use std::process::exit;
use std::str::FromStr;

use log::error;

use crate::data::action::Execute;
use crate::state::MONITORS;
use crate::win_api::monitor::get_monitor_from_window;
use crate::win_api::window::{get_foreground_handle, get_foreground_window};

#[derive(Clone, PartialEq)]
pub struct MoveToWorkspace {
    pub id: u32,
}

impl Execute for MoveToWorkspace {
    fn execute(&self) {
        let mut monitors = MONITORS.lock().unwrap();
        let window_handle = get_foreground_handle();
        let monitor_handle = get_monitor_from_window(window_handle);
        let monitor = monitors
            .iter_mut()
            .find(|monitor| monitor.hmonitor == monitor_handle)
            .expect("Unable to get current monitor");
        let window = get_foreground_window();
        monitor.remove_window(&window);
        window.minimize();
        monitor.add_window_to_workspace(self.id, &window);
        monitor.current_workspace().arrange_windows();
        // let mut monitors = MONITORS.lock().unwrap();
        // let window_handle = get_foreground_handle();
        // let monitor_handle = get_monitor_from_window(window_handle);
        // let mut monitor = monitors
        //     .iter()
        //     .find(|monitor| monitor.hmonitor == monitor_handle)
        //     .expect("Unable to get current monitor")
        //     .clone();
        // let mut workspaces = monitor.workspaces.clone();
        // let mut target_workspace = monitor.get_workspace(self.id);
        // let current_window = get_foreground_window();
        // let mut current_workspace = monitor
        //     .workspace_from_window(&current_window)
        //     .expect("Unable to find workspace for window");
        // if current_workspace.id == target_workspace.id {
        //     debug!("Skipping request to move window to current workspace");
        //     return;
        // }
        // current_workspace.remove_window(&current_window);
        // workspaces[(current_workspace.id - 1) as usize] = current_workspace.clone();
        // target_workspace.add_window(&current_window);
        // workspaces[(&self.id - 1) as usize] = target_workspace.clone();
        // monitor.workspaces = workspaces;
        // let monitor_index = monitors
        //     .iter()
        //     .position(|mon| mon.hmonitor == monitor.hmonitor)
        //     .expect("Unable to find stateful index of monitor");
        // monitors[monitor_index] = monitor;
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
