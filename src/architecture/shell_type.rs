
#[derive(Debug, Clone, Copy)]
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