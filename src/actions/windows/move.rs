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
        let hwnd = get_foreground_handle();
        let monitor_ref = Monitor::current();
        let mut monitor = monitor_ref.borrow_mut();
        monitor.move_window_in_direction(&hwnd, &self.direction);
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
