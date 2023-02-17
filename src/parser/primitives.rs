use nom::{
    IResult, 
    combinator::map,
    branch::alt,
    character::complete::{i32, anychar, one_of, alphanumeric1},
    number::complete::float, 
};

use crate::architecture::{shell_data::ShellData, shell_type::ShellType};

pub fn parse_shell_data(data_types: Vec<ShellType>) -> impl Fn(&str)-> IResult<&str, ShellData> {
    move |input| {
        for ty in &data_types {
            match ty {
                ShellType::Int => return parse_int(input),
                ShellType::Float => return parse_float(input),
                ShellType::Number => return parse_number(input),
                ShellType::String => return parse_string(input),
                ShellType::Any => return parse_string(input), //CHANGE TO ANY OF THE VALUES NOT STRING
                ShellType::FilePath => return parse_file_path(input),
            }
        }
        println!("Error case in shell data parsing");
        Err(nom::Err::Failure(nom::error::Error::new("No shell types given", nom::error::ErrorKind::Fail)))
    }
}

pub fn parse_file_path(input: &str) -> IResult<&str, ShellData> {
    //TODO: PARSE ONLY FILE PTH VALID CHARACTERS
    parse_string(input)
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
    map(alphanumeric1, |v: &str| ShellData::String(v.to_string()))(input)
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
}