use std::str::FromStr;

use log::debug;

use crate::data::action::Execute;
use crate::data::window::Window;
use crate::state::MONITORS;
use crate::win_api::window::get_foreground_window;

#[derive(Clone, PartialEq)]
pub struct Focus {
    pub direction: String,
}

impl Execute for Focus {
    fn execute(&self) {
        debug!("Found monitors {:?}", MONITORS.lock().unwrap());
        let current = get_foreground_window();
        let target: Window = current.find_nearest_in_direction(&self.direction);
        target.focus();
    }
}

impl FromStr for Focus {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_ascii_uppercase().as_str() {
            "FOCUS_LEFT" => Ok(Focus {
                direction: "left".to_string(),
            }),
            "FOCUS_DOWN" => Ok(Focus {
                direction: "down".to_string(),
            }),
            "FOCUS_UP" => Ok(Focus {
                direction: "up".to_string(),
            }),
            "FOCUS_RIGHT" => Ok(Focus {
                direction: "right".to_string(),
            }),
            _ => Err(()),
        }
    }
}

impl std::fmt::Debug for Focus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Focus {}", self.direction)
    }
}
