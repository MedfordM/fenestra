use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
pub enum WindowManagerAction {
    LEFT,
    DOWN,
    UP,
    RIGHT,
}

impl FromStr for WindowManagerAction {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_ascii_uppercase().as_str() {
            "LEFT" => Ok(WindowManagerAction::LEFT),
            "DOWN" => Ok(WindowManagerAction::DOWN),
            "UP" => Ok(WindowManagerAction::UP),
            "RIGHT" => Ok(WindowManagerAction::RIGHT),
            _ => Err(()),
        }
    }
}

impl std::fmt::Debug for WindowManagerAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowManagerAction::LEFT => write!(f, "LEFT"),
            WindowManagerAction::DOWN => write!(f, "DOWN"),
            WindowManagerAction::UP => write!(f, "UP"),
            WindowManagerAction::RIGHT => write!(f, "RIGHT"),
        }
    }
}