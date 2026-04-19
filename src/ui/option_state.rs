use std::collections::HashSet;
use crate::service::option_parser::TuistOption;

#[derive(Clone)]
pub struct OptionState {
    pub options: Vec<TuistOption>,
    pub selected: HashSet<usize>,
    pub cursor: usize,
    pub warning: Option<String>,
}

impl OptionState {
    pub fn new(items: Vec<TuistOption>) -> OptionState {
        OptionState {
            options: items,
            selected: HashSet::new(),
            cursor: 0,
            warning: None,
        }
    }
}

