use std::fmt::{Debug, Formatter};
use crate::data::window::Window;

#[derive(Clone)]
struct Workspace {
    pub id: u32,
    pub windows: Vec<Window>
}

impl Debug for Workspace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.id, self.windows)
    }
}
