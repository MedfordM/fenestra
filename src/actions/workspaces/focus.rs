use crate::data::action::Execute;

#[derive(Clone, PartialEq)]
pub struct FocusWorkspace {
    pub id: String,
}

impl Execute for FocusWorkspace {
    fn execute(&self) {
    }
}

// impl FromStr for FocusWorkspace {
//     type Err = ();
//     fn from_str(input: &str) -> Result<Self, Self::Err> {
//         match input.to_ascii_uppercase().as_str() {
//             _ => Err(()),
//         }
//     }
// }
