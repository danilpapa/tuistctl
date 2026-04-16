use std::collections::HashSet;
use crate::service::option_parser::TuistOption;

#[derive(Clone)]
pub struct OptionState {
    pub options: Vec<TuistOption>,
    pub selected: HashSet<usize>,
    pub cursor: usize,
}

impl OptionState {
    pub fn new(items: Vec<TuistOption>) -> OptionState {
        OptionState {
            options: items,
            selected: HashSet::new(),
            cursor: 0,
        }
    }
}

