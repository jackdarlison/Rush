use nom::{
    IResult, 
    combinator::{map, verify},
    branch::alt,
    character::{complete::{i32, alphanumeric1, multispace1, anychar}, is_space},
    number::complete::float,
    bytes::complete::tag,
    multi::fold_many1,
    combinator::not,
};

use crate::architecture::{shell_data::ShellData, shell_type::ShellType};

pub fn parse_shell_data(data_type: ShellType) -> impl Fn(&str) -> IResult<&str, ShellData> {
    move |input| {
        match data_type {
            ShellType::Int => return parse_int(input),
            ShellType::Float => return parse_float(input),
            ShellType::Number => return parse_number(input),
            ShellType::String => return parse_string(input),
            ShellType::Any => return parse_string(input), //CHANGE TO ANY OF THE VALUES NOT STRING
            ShellType::FilePath => return parse_file_path(input), 
        }
    }
}

pub fn parse_shell_data_many(data_types: Vec<ShellType>) -> impl Fn(&str) -> IResult<&str, ShellData> {
    move |input| {
        for ty in &data_types {
            return parse_shell_data(*ty)(input)
        }
        Err(nom::Err::Failure(nom::error::Error::new("No shell types given", nom::error::ErrorKind::Fail)))
    }
}

pub fn file_path_character(input: &str) -> IResult<&str, &str> {
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

pub fn parse_file_path(input: &str) -> IResult<&str, ShellData> {
    let (rest, filepath) = fold_many1(
        file_path_character,
        || String::new(),
         | mut acc, file_char| {
            acc.push_str(file_char);
            acc
         }
    )(input)?;
    //TODO: Match glob pattern struct to check valid path
    return Ok((rest, ShellData::FilePath(filepath)))
}

pub fn parse_int(input: &str) -> IResult<&str, ShellData> {
    map(i32, |v| ShellData::Int(v))(input)
}

pub fn parse_float(input: &str) -> IResult<&str, ShellData> {
    map(float, |v| ShellData::Float(v))(input)
}

pub fn parse_number(input: &str) -> IResult<&str, ShellData> {
    parse_float(input)
    //potential to tell if number is actually an integer
}

pub fn parse_string(input: &str) -> IResult<&str, ShellData> {
    //TODO: Parse any character not just alphanum
    let (rest, string) = fold_many1(
        verify(anychar, |c| !c.is_ascii_whitespace()),
        || String::new(),
        |mut acc, ch| {
            acc.push(ch);
            acc
    })(input)?;
    Ok((rest, ShellData::String(string)))

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int() {
        assert_eq!(parse_int("5"), Ok(("", ShellData::Int(5))))
    }

    #[test]
    fn test_number_float() {
        assert_eq!(parse_number("5.0"), Ok(("", ShellData::Float(5.0))))
    }

    #[test]
    fn test_string() {
        assert_eq!(parse_string("-input[]/..* abc"), Ok((" abc", ShellData::String(String::from("-input[]/..*")))))
    }
}