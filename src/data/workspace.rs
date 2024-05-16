pub struct Workspace {
    pub index: usize,
    pub groups: Vec<usize>,
    pub active: bool,
}

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
