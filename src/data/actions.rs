use std::str::FromStr;

use crate::util;

pub trait Execute {
    fn execute(&self);
}

#[derive(Clone, PartialEq)]
pub struct Focus {
    pub direction: String,
}

impl Execute for Focus {
    fn execute(&self) {
        util::windows::focus::focus_direction(self);
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

#[derive(Clone, PartialEq)]
pub enum WindowManagerAction {
    Focus(Focus),
}

impl Execute for WindowManagerAction {
    fn execute(&self) {
        return match self {
            WindowManagerAction::Focus(focus) => focus.execute(),
        };
    }
}

impl FromStr for WindowManagerAction {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.to_ascii_uppercase().contains("FOCUS") {
            let result = Focus::from_str(input);
            if result.is_ok() {
                return Ok(WindowManagerAction::Focus(result.unwrap()));
            }
        }
        return Err(());
    }
}

impl std::fmt::Debug for WindowManagerAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            WindowManagerAction::Focus(focus) => focus.fmt(f),
        };
    }
}
