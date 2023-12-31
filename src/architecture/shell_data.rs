use std::{fmt, ops::Range};


/// Contains the different styles of structured data present in the shell 
#[derive(Debug, Clone, PartialEq)]
pub enum ShellData {
    FilePath(String),
    String(String),
    Float(f32),
    Int(i32),
    Range(Range<i32>)
}

impl fmt::Display for ShellData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellData::FilePath(m) => write!(f, "{}", m),
            ShellData::String(m) => write!(f, "{}", m),
            ShellData::Float(m) => write!(f, "{}", m),
            ShellData::Int(m) => write!(f, "{}", m),
            ShellData::Range(r) => write!(f, "{}..<{}", r.start, r.end),
        }
    } 
}
