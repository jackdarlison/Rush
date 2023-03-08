use std::error::Error;
use std::fmt;



//Error type for the shell
#[derive(Debug, Clone)]
pub enum ShellError {
    InputError,
    CommandError,
    UnknownError,
    DataTypeError,
}

impl Error for ShellError {

}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There has been a shell error")
    }
}
