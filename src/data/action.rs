use std::str::FromStr;

use crate::actions::windows::focus::FocusWindow;
use crate::actions::windows::r#move::MoveWindow;

#[derive(Clone, PartialEq)]
pub enum WindowManagerAction {
    FocusWindow(FocusWindow),
    MoveWindow(MoveWindow)
}

impl Execute for WindowManagerAction {
    fn execute(&self) {
        return match self {
            WindowManagerAction::FocusWindow(focus) => focus.execute(),
            WindowManagerAction::MoveWindow(focus) => focus.execute(),
        };
    }
}

impl FromStr for WindowManagerAction {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.to_ascii_uppercase().contains("FOCUS") {
            let result = FocusWindow::from_str(input);
            if result.is_ok() {
                return Ok(WindowManagerAction::FocusWindow(result.unwrap()));
            }
        } else if input.to_ascii_uppercase().contains("FOCUS") {
            let result = MoveWindow::from_str(input);
            if result.is_ok() {
                return Ok(WindowManagerAction::MoveWindow(result.unwrap()));
            }
        }
        return Err(());
    }
}

impl std::fmt::Debug for WindowManagerAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            WindowManagerAction::FocusWindow(focus) => focus.fmt(f),
        };
    }
}

pub trait Execute {
    fn execute(&self);
}
