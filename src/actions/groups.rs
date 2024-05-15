use crate::data::action::Action;
use crate::data::common::axis::Axis;
use crate::state::management::state_manager::StateManager;
use log::error;
use std::process::exit;
use std::str::FromStr;

pub struct SetSplitAxis {
    pub axis: Axis,
}

impl Action for SetSplitAxis {
    fn execute(&self, state_manager: &mut StateManager) {
        state_manager.setSplitAxis(self.axis);
    }
}

impl FromStr for SetSplitAxis {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input_up = input.to_ascii_uppercase();
        if !input_up.contains("SPLIT_ON_AXIS_") {
            return Err(());
        }
        let axis_str = input_up.strip_prefix("SPLIT_ON_AXIS_").unwrap();
        let axis = Axis::from_str(axis_str);
        if axis.is_err() {
            error!("Unable to parse axis from {}", &axis_str);
            exit(100);
        }
        Ok(SetSplitAxis {
            axis: axis.unwrap(),
        })
    }
}
