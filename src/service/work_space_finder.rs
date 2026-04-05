use std::ffi::OsStr;
use walkdir::DirEntry;
use std::path::{PathBuf};
use walkdir::WalkDir;

pub fn find_workspace() -> Option<PathBuf> {
    // TODO: exclude .gitignore files
    for entry in WalkDir::new(".") {
        let entry = entry
            .expect("Could not scan files in directory");
        if match_workspace(&entry) {
            return Some(entry.into_path());
        }
    }
    return None
}

fn match_workspace(entry: &DirEntry) -> bool {
    entry.file_type().is_dir() &&
        entry.path().extension() == Some(OsStr::new("xcworkspace"))
}