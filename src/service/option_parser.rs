use std::{fmt, fs};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct TuistOption {
    pub name: String,
    pub exec: Option<String>,
    pub tip: Option<String>,
}

impl fmt::Display for TuistOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct TuistOptionsList {
    pub items: Vec<TuistOption>,
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
        .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with("//"))
        .map(|line| {
            let (definition, tip) = if let Some(idx) = line.find("//") {
                let tip = line[idx + 2..].trim().to_string();
                (&line[..idx], Some(tip))
            } else {
                (line, None)
            };
            let definition = definition.trim();

            if definition.contains("=") {
                let params = definition.splitn(2, "=").collect::<Vec<&str>>();
                TuistOption {
                    name: params[0].trim().to_string(),
                    exec: Some(params[1].trim().to_string()),
                    tip,
                }
            } else {
                TuistOption { name: definition.to_string(), exec: None, tip }
            }
        })
        .collect::<TuistOptionsList>()
}