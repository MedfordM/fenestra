use std::str::FromStr;

use crate::data::action::Execute;
use crate::data::common::direction::Direction;
use crate::data::monitor::Monitor;
use crate::win_api::window::get_foreground_handle;

#[derive(Clone, PartialEq)]
pub struct MoveWindow {
    pub direction: Direction,
}

impl Execute for MoveWindow {
    fn execute(&self) {
        let current_hwnd = get_foreground_handle();
        let current_monitor_ref = Monitor::current();
        let mut current_monitor = current_monitor_ref.borrow_mut();
        let current_workspace = current_monitor.current_workspace();
        {
            let current_group = current_workspace.current_group();
            let mut group_windows = current_group.get_windows().borrow_mut();
            let current_window = group_windows
                .iter_mut()
                .find(|window| window.hwnd == current_hwnd)
                .unwrap();
            // let mut current_window = get_foreground_window();
            current_window.move_in_direction(&self.direction);
        }
        current_workspace.arrange_windows();
    }
}

impl FromStr for MoveWindow {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input_up: String = input.to_ascii_uppercase();
        if !input_up.contains("MOVE_WINDOW_") {
            return Err(());
        }
        let direction_str: &str = input_up.strip_prefix("MOVE_WINDOW_").unwrap();
        let direction = Direction::from_str(direction_str);
        if direction.is_err() {
            return Err(());
        }
        Ok(MoveWindow {
            direction: direction.unwrap(),
        })
    }
}

impl std::fmt::Debug for MoveWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Move window {}", self.direction)
    }
}
