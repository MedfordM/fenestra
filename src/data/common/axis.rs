use crate::data::common::axis::Axis::{HORIZONTAL, VERTICAL};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq)]
pub enum Axis {
    HORIZONTAL,
    VERTICAL,
}

impl FromStr for Axis {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_ascii_uppercase().as_str() {
            "HORIZONTAL" => Ok(HORIZONTAL),
            "VERTICAL" => Ok(VERTICAL),
            _ => Err(()),
        }
    }
}

impl Display for Axis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            HORIZONTAL => String::from("horizontal"),
            VERTICAL => String::from("vertical"),
        };
        write!(f, "{}", str)
    }
}

impl Debug for Axis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
