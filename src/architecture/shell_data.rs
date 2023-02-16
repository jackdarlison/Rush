
#[derive(Debug, Clone, PartialEq)]
pub enum ShellData {
    FilePath(String),
    DirPath(String),
    GlobPath(String),
    String(String),
    Float(f32),
    Int(i32),
}
