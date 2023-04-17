
/// Contains information regarding the expected types a command can take
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