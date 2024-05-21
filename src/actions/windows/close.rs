use crate::data::action::Action;
use crate::state::management::state_manager::StateManager;
use std::str::FromStr;

pub struct CloseWindow {}

impl Action for CloseWindow {
    fn execute(&self, state_manager: &mut StateManager) {
        state_manager.close_window();
    }
}

impl FromStr for CloseWindow {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input_up: String = input.to_ascii_uppercase();
        if !input_up.contains("CLOSE_WINDOW") {
            return Err(());
        }
        Ok(CloseWindow {})
    }
}
