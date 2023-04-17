
use std::ops::Range;

use nom::{
    IResult, 
    combinator::map,
    branch::alt,
    character::{complete::{char, i32, alphanumeric1, anychar, oct_digit1}},
    number::complete::float,
    bytes::complete::tag,
    multi::{fold_many1, many_till}, Err::{Error, Failure},
    Err, sequence::tuple,
};

use crate::{architecture::{shell_data::ShellData, shell_type::ShellType}, convert_parser_data_error};

use super::parser_error::ParserError;

/// Returns a parser for a given [`ShellType`]
pub fn parse_shell_data(data_type: ShellType) -> impl Fn(&str) -> IResult<&str, ShellData, ParserError<&str>> {
    move |input| {
        match data_type {
            ShellType::Int => return parse_int(input),
            ShellType::Float => return parse_float(input),
            ShellType::Number => return parse_number(input),
            ShellType::String => return parse_string(input),
            ShellType::Any => return parse_string(input), //CHANGE TO ANY OF THE VALUES NOT STRING
            ShellType::FilePath => return parse_file_path(input), 
            ShellType::Octal => return parse_octal(input),
        }
    }
}

/// Returns a parser for a list of [`ShellType`]s
pub fn parse_shell_data_many(data_types: Vec<ShellType>) -> impl Fn(&str) -> IResult<&str, ShellData, ParserError<&str>> {
    move |input| {
        for ty in data_types.clone() {
            if let Ok(r) = parse_shell_data(ty)(input) {
                return Ok(r)
            }
        }
        Err(Error(ParserError::DataError(data_types.clone())))
    }
}

/// Parses a character that is valid within a filepath
/// 
/// 0-9, a-z, A-Z, /, -, _, ., ?, *, \[, \], !, \\ 
pub fn file_path_character(input: &str) -> IResult<&str, &str, ParserError<&str>> {
    alt((
        alphanumeric1,
        tag("/"),
        tag("-"),
        tag("_"),
        tag("."),
        tag("?"),
        tag("*"),
        tag("["),
        tag("]"),
        tag("!"),
        tag("\\ "),
    ))(input)
}

/// Parses a valid file path, using [`file_path_character`]
pub fn parse_file_path(input: &str) -> IResult<&str, ShellData, ParserError<&str>> {
    let (rest, filepath) = convert_parser_data_error!(fold_many1(
        file_path_character,
        || String::new(),
         | mut acc, file_char| {
            acc.push_str(file_char);
            acc
         }
    )(input), vec![ShellType::FilePath])?;
    //TODO: Match glob pattern struct to check valid path
    return Ok((rest, ShellData::FilePath(filepath)))
}

/// Parses an integer
pub fn parse_int(input: &str) -> IResult<&str, ShellData, ParserError<&str>> {
    convert_parser_data_error!(map(i32, |v| ShellData::Int(v))(input), vec![ShellType::Int])
}

/// Parses a float
pub fn parse_float(input: &str) -> IResult<&str, ShellData, ParserError<&str>> {
    convert_parser_data_error!(map(float, |v| ShellData::Float(v))(input), vec![ShellType::Float])
}

/// Parses either a float or an integer
pub fn parse_number(input: &str) -> IResult<&str, ShellData, ParserError<&str>> {
    //TODO: Decide if number is float or not
    convert_parser_data_error!(parse_float(input), vec![ShellType::Float])
}

/// Parses an octal
pub fn parse_octal(input: &str) -> IResult<&str, ShellData, ParserError<&str>> {
    convert_parser_data_error!(map(oct_digit1, |v| {
        ShellData::Int(i32::from_str_radix(v, 8).unwrap())
    })(input), vec![ShellType::Octal])
}

/// Parses a string: any characters between two quotes
pub fn parse_string(input: &str) -> IResult<&str, ShellData, ParserError<&str>> {
    let (rest, _) = convert_parser_data_error!(char('"')(input), vec![ShellType::String])?;
    let (rest, (chars, _)) = convert_parser_data_error!(many_till(anychar, char('"'))(rest), vec![ShellType::String])?;
    let string = chars.iter().fold(String::new(), |mut acc, c| {acc.push(*c); acc});
    Ok((rest, ShellData::String(string)))
}

/// Parses a range, inclusive below and either above
/// 
/// x..<y is exclusive above, x..=y is inclusive above
pub fn parse_range(input: &str) -> IResult<&str, Range<i32>, ParserError<&str>> {
    let (rest, (s, _, inc, e)) = tuple((i32, tag(".."), alt((tag("<"), tag("="))), i32))(input)?;
    if inc == "=" {
        Ok((rest, Range {start: s, end: e+1}))
    } else {
        Ok((rest, Range {start: s, end: e}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filepath() {
        assert_eq!(parse_file_path("valid/**/file\\ path"), Ok(("", ShellData::FilePath("valid/**/file\\ path".to_string()))));
        assert_eq!(parse_file_path("partial/&&/filepath"), Ok(("&&/filepath", ShellData::FilePath("partial/".to_string()))));
    }

    #[test]
    fn test_int() {
        assert_eq!(parse_int("5"), Ok(("", ShellData::Int(5))))
    }

    #[test]
    fn test_number_float() {
        assert_eq!(parse_number("5.0"), Ok(("", ShellData::Float(5.0))))
    }

    #[test]
    fn test_octal() {
        assert_eq!(parse_octal("777"), Ok(("", ShellData::Int(0o777))));
        assert_eq!(parse_octal("999"), Err(Error(ParserError::DataError(vec![ShellType::Octal]))));
    }

    #[test]
    fn test_string() {
        assert_eq!(parse_string("\"test string 123$\""), Ok(("", ShellData::String(String::from("test string 123$")))))
    }

    #[test]
    fn test_range() {
        assert_eq!(parse_range("10..<20"), Ok(("", Range {start: 10, end: 20})));
        assert_eq!(parse_range("10..=20"), Ok(("", Range {start: 10, end: 21})));
    }
}