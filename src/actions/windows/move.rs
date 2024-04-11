use std::str::FromStr;

use crate::data::action::Execute;
use crate::win_api::window::get_foreground_window;

#[derive(Clone, PartialEq)]
pub struct Move {
    pub direction: String,
}

impl Execute for Move {
    fn execute(&self) {
        let current = get_foreground_window();
    }
}

impl FromStr for Move {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_ascii_uppercase().as_str() {
            "MOVE_LEFT" => Ok(Move {
                direction: "left".to_string(),
            }),
            "MOVE_DOWN" => Ok(Move {
                direction: "down".to_string(),
            }),
            "MOVE_UP" => Ok(Move {
                direction: "up".to_string(),
            }),
            "MOVE_RIGHT" => Ok(Move {
                direction: "right".to_string(),
            }),
            _ => Err(()),
        }
    }
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Move {}", self.direction)
    }
}
