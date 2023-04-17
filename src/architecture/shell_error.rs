use std::error::Error;
use std::fmt;



/// Contains the different styles of errors present within commands
#[derive(Debug, Clone)]
pub enum ShellError {
    InputError(String),
    CommandError(String),
    UnknownError(String),
    DataTypeError(String),
    None,
}

impl Error for ShellError {}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let binding = "".to_string();
        let msg = match self {
            ShellError::InputError(m) => m,
            ShellError::CommandError(m) => m,
            ShellError::UnknownError(m) => m,
            ShellError::DataTypeError(m) => m, 
            ShellError::None => &binding,
        };
        write!(f, "{}", msg)
    }
}
