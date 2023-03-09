use std::{path::PathBuf, fs::canonicalize};


pub fn name(path: &PathBuf) -> String {
    let path = canonicalize(path).unwrap();
    String::from(path.to_str().unwrap_or("Error getting path name"))
}

pub fn hidden(path: &PathBuf) -> bool {
    components(path).last().and_then(|s| Some(s.starts_with("."))).unwrap_or(true)
}

pub fn components(path: &PathBuf) -> Vec<String> {
    path.to_str().unwrap_or("").split("/").map(|s| String::from(s)).collect()
}