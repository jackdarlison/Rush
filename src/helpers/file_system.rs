use std::{path::PathBuf, fs::canonicalize};


pub fn name(path: &PathBuf) -> String {
    if let Ok(canon_dir) = canonicalize(path) {
        String::from(canon_dir.to_str().unwrap_or("Error getting canon directory name"))
    } else {
        String::from(path.to_str().unwrap_or("Error getting path string"))
    }
}

pub fn hidden(path: &PathBuf) -> bool {
    components(path).last().and_then(|s| Some(s.starts_with("."))).unwrap_or(true)
}

pub fn components(path: &PathBuf) -> Vec<String> {
    path.to_str().unwrap_or("").split("/").map(|s| String::from(s)).collect()
}