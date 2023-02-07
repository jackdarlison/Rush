
#[derive(Debug, Clone)]
pub enum ShellType {
    Any,
    FilePath,
    DirPath,
    GlobPath,
    String,
    Number,
    Float,
    Int,
}