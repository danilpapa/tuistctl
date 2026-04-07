use std::ffi::OsStr;
use walkdir::DirEntry;
use std::path::{PathBuf};
use walkdir::WalkDir;

pub fn find_workspace() -> Option<PathBuf> {
    for entry in WalkDir::new(".") {
        let entry = entry
            .expect("Could not scan files in directory");
        if match_workspace(&entry) {
            return Some(entry.into_path());
        }
    }
    None
}

pub fn options_file() -> Option<PathBuf> {
    for entry in WalkDir::new(".") {
        let entry = entry
            .expect("Could not scan files in directory");
        if match_workspace(&entry) {
            return Some(entry.into_path());
        }
    };
    None
}

fn match_workspace(entry: &DirEntry) -> bool {
    entry.file_type().is_dir() &&
        entry.path().extension() == Some(OsStr::new("xcworkspace"))
}

fn match_options(entry: &DirEntry) -> bool {
    entry.file_type().is_file() &&
        entry.path().extension() == Some(OsStr::new("txt"))
}