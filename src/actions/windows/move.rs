use std::str::FromStr;

use log::debug;

use crate::data::action::Execute;
use crate::data::common::direction::Direction;
use crate::data::window::Window;
use crate::win_api::window::get_foreground_window;

#[derive(Clone, PartialEq)]
pub struct MoveWindow {
    pub direction: Direction,
}

impl Execute for MoveWindow {
    fn execute(&self) {
        let mut current: Window = get_foreground_window();
        let target: Window = current.find_nearest_in_direction(&self.direction);
        if target != current {
            debug!("Swapping window {} with {}", current.title, target.title);
            current.swap_windows(target);
        } else {
            debug!("Moving window {} {}", current.title, &self.direction);
        }
    }
}

impl FromStr for MoveWindow {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input_up: String = input.to_ascii_uppercase();
        if !input_up.contains("MOVE_") {
            return Err(());
        }
        let direction_str: &str = input_up.strip_prefix("MOVE_").unwrap();
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
        write!(f, "Move {}", self.direction)
    }
}
