use crate::data::action::Action;
use std::str::FromStr;

use crate::data::common::direction::Direction;
use crate::state::management::state_manager::StateManager;

pub struct MoveWindow {
    pub direction: Direction,
}

impl Action for MoveWindow {
    fn execute(&self, state_manager: &mut StateManager) {
        state_manager.move_window_in_direction(self.direction.clone())
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
