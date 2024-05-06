pub struct Workspace {
    pub index: usize,
    pub groups: Vec<usize>,
    pub active: bool
}

impl Workspace {
    pub fn new(index: usize) -> Workspace {
        Workspace {
            index,
            groups: Vec::new(),
            active: index == 0
        }
    }
}

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
