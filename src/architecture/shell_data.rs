use std::fmt;


#[derive(Debug, Clone, PartialEq)]
pub enum ShellData {
    FilePath(String),
    DirPath(String),
    GlobPath(String),
    String(String),
    Float(f32),
    Int(i32),
}

impl fmt::Display for ShellData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellData::FilePath(m) => write!(f, "{}", m),
            ShellData::DirPath(m) => write!(f, "{}", m),
            ShellData::GlobPath(m) => write!(f, "{}", m),
            ShellData::String(m) => write!(f, "{}", m),
            ShellData::Float(m) => write!(f, "{}", m),
            ShellData::Int(m) => write!(f, "{}", m),
        }
    } 
}
