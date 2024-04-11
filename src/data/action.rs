use std::str::FromStr;
use crate::actions::windows::focus::Focus;

pub trait Execute {
    fn execute(&self);
}


pub struct Workspace {
    id: String
}

impl Execute for Workspace {
    fn execute(&self) {
        
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
