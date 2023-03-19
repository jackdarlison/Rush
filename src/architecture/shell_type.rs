
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShellType {
    Any,
    FilePath,
    String,
    Number,
    Float,
    Int,
    Octal,
}