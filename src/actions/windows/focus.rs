use std::str::FromStr;

use crate::data::action::Execute;
use crate::data::common::direction::Direction;
use crate::data::window::Window;
use crate::win_api::window::get_foreground_window;

#[derive(Clone, PartialEq)]
pub struct FocusWindow {
    pub direction: Direction,
}

impl Execute for FocusWindow {
    fn execute(&self) {
        let current = get_foreground_window();
        let target: Window = current.find_nearest_in_direction(&self.direction);
        target.focus();
    }
}

impl FromStr for FocusWindow {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input_up: String = input.to_ascii_uppercase();
        if !input_up.contains("FOCUS_") {
            return Err(());
        }
        let direction_str: &str = input_up.strip_prefix("FOCUS_").unwrap();
        let direction = Direction::from_str(direction_str);
        if direction.is_err() {
            return Err(());
        }
        Ok(FocusWindow {
            direction: direction.unwrap(),
        })
    }
}

impl std::fmt::Debug for FocusWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Focus {}", self.direction)
    }
}
