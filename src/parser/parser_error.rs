use nom::error::{ErrorKind, ParseError};

use crate::architecture::shell_type::ShellType;


#[derive(Debug, PartialEq)]
pub enum ParserError<I> {
    CommandError(String),
    OptionError(String),
    ArgumentError(String),
    DataError(ShellType),
    Incomplete,
    Unknown,
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for ParserError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        ParserError::Nom(input, kind)
    }

    fn append(input: I, kind: ErrorKind, other: Self) -> Self {
        other
    }
}