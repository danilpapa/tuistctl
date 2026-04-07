use std::fs;
use std::path::Path;

pub struct TuistOption {
    pub name: String,
    pub exec: Option<String>,
}

pub struct TuistOptionsList {
    items: Vec<TuistOption>,
}

impl TuistOptionsList {
    pub fn iter(&self) -> impl Iterator<Item = &TuistOption> {
        self.items.iter()
    }
}

impl FromIterator<TuistOption> for TuistOptionsList {
    fn from_iter<T: IntoIterator<Item=TuistOption>>(iter: T) -> Self {
        Self {
            items: iter.into_iter().collect()
        }
    }
}

pub fn get_options(path: &Path) -> TuistOptionsList {
    fs::read_to_string(&path)
        .expect("Failed to read options!")
        .lines()
        .map(|line| line.to_string())
        .map(|line| {
            if line.contains("=") {
                let params = line.split("=").collect::<Vec<&str>>();
                return TuistOption {
                    name: params[0].to_string(),
                    exec: Some(params[1].to_string()),
                }
            } else {
                return TuistOption { name: line, exec: None }
            }
        })
        .collect::<TuistOptionsList>()
}