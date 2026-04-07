use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn get_options(path: &Path) -> Vec<String> {
    fs::read_to_string(&path)
        .expect("Failed to read options!")
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}