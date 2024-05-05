pub struct Workspace {
    pub index: usize,
    pub focused: bool,
    pub groups: Vec<usize>,
}

impl Workspace {
    pub fn new(index: usize) -> Workspace {
        let focused = match index {
            0 => true,
            _ => false,
        };
        Workspace {
            index,
            focused,
            groups: Vec::new()
        }
    }
}

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
