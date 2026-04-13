use std::collections::HashSet;

pub struct TargetsState {
    pub targets: Vec<String>,
    pub selected: HashSet<usize>,
    pub cursor: usize,
}

impl TargetsState {
    pub fn new(items: Vec<String>) -> Self {
        TargetsState {
            targets: items,
            selected: HashSet::new(),
            cursor: 0,
        }
    }
}