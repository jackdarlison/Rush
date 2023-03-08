
#[derive(Debug, Clone, Copy)]
pub enum ShellType {
    Any,
    FilePath,
    String,
    Number,
    Float,
    Int,
    Octal,
}