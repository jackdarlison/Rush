use std::{path::PathBuf, fs::canonicalize};
use std::fs;


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

pub fn read_file_contents(path: &PathBuf) -> std::io::Result<String> {
    fs::read_to_string(path).map(preprocess)
}

fn preprocess(file_contents: String) -> String {
    //Any preprocessing goes in here
    file_contents
}