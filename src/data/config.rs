use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
pub enum WindowManagerAction {
    FocusLeft,
    FocusDown,
    FocusUp,
    FocusRight,
}

impl FromStr for WindowManagerAction {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_ascii_uppercase().as_str() {
            "FOCUS_LEFT" => Ok(WindowManagerAction::FocusLeft),
            "FOCUS_DOWN" => Ok(WindowManagerAction::FocusDown),
            "FOCUS_UP" => Ok(WindowManagerAction::FocusUp),
            "FOCUS_RIGHT" => Ok(WindowManagerAction::FocusRight),
            _ => Err(()),
        }
    }
}

impl std::fmt::Debug for WindowManagerAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowManagerAction::FocusLeft => write!(f, "FocusLeft"),
            WindowManagerAction::FocusDown => write!(f, "FocusDown"),
            WindowManagerAction::FocusUp => write!(f, "FocusUp"),
            WindowManagerAction::FocusRight => write!(f, "FocusRight"),
        }
    }
}
