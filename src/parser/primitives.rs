
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

use crate::{architecture::{shell_data::ShellData, shell_type::ShellType}, convert_parser_error};

use super::parser_error::ParserError;

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

pub fn parse_shell_data_many(data_types: Vec<ShellType>) -> impl Fn(&str) -> IResult<&str, ShellData, ParserError<&str>> {
    move |input| {
        for ty in &data_types {
            return parse_shell_data(*ty)(input)
        }
        Err(Error(ParserError::Unknown))
    }
}

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

pub fn parse_file_path(input: &str) -> IResult<&str, ShellData, ParserError<&str>> {
    let (rest, filepath) = convert_parser_error!(fold_many1(
        file_path_character,
        || String::new(),
         | mut acc, file_char| {
            acc.push_str(file_char);
            acc
         }
    )(input), ShellType::FilePath)?;
    //TODO: Match glob pattern struct to check valid path
    return Ok((rest, ShellData::FilePath(filepath)))
}

pub fn parse_int(input: &str) -> IResult<&str, ShellData, ParserError<&str>> {
    convert_parser_error!(map(i32, |v| ShellData::Int(v))(input), ShellType::Int)
}

pub fn parse_float(input: &str) -> IResult<&str, ShellData, ParserError<&str>> {
    convert_parser_error!(map(float, |v| ShellData::Float(v))(input), ShellType::Float)
}

pub fn parse_number(input: &str) -> IResult<&str, ShellData, ParserError<&str>> {
    //Decide if number is float or not
    convert_parser_error!(parse_float(input), ShellType::Float)
}

pub fn parse_octal(input: &str) -> IResult<&str, ShellData, ParserError<&str>> {
    convert_parser_error!(map(oct_digit1, |v| {
        ShellData::Int(i32::from_str_radix(v, 8).unwrap())
    })(input), ShellType::Octal)
}

pub fn parse_string(input: &str) -> IResult<&str, ShellData, ParserError<&str>> {
    let (rest, _) = convert_parser_error!(char('"')(input), ShellType::String)?;
    let (rest, (chars, _)) = convert_parser_error!(many_till(anychar, char('"'))(rest), ShellType::String)?;
    let string = chars.iter().fold(String::new(), |mut acc, c| {acc.push(*c); acc});
    Ok((rest, ShellData::String(string)))
}

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
        assert_eq!(parse_octal("999"), Err(Error(ParserError::DataError(ShellType::Octal))));
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