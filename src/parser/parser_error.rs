use std::fmt;

use nom::error::{ErrorKind, ParseError};

use crate::architecture::shell_type::ShellType;


#[derive(Debug, PartialEq)]
pub enum ParserError<I> {
    CommandError(String),
    OptionError(String),
    ArgumentError(String),
    DataError(Vec<ShellType>),
    Incomplete,
    Unknown(String),
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for ParserError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        ParserError::Nom(input, kind)
    }

    fn append(_input: I, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I: std::fmt::Debug> fmt::Display for ParserError<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::CommandError(c) => write!(f, "{}", c),
            ParserError::OptionError(o) => write!(f, "{}", o),
            ParserError::ArgumentError(a) => write!(f, "{}", a),
            ParserError::DataError(ts) => write!(f, "Invalid data type, expected type from {:?}", ts),
            ParserError::Incomplete => write!(f, "Incomplete parsing"),
            ParserError::Unknown(s) => write!(f, "An unknown error has occured: {}", s),
            nom => write!(f, "{:?}", nom),
        }
    }
}