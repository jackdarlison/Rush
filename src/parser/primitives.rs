use nom::{
    IResult, 
    combinator::map,
    branch::alt,
    character::complete::{i32, anychar, one_of},
    number::complete::float, 
};

use crate::architecture::shell_data::ShellData;


fn parse_file_path(input: &str) -> IResult<&str, ShellData> {
    todo!()
}

fn parse_int(input: &str) -> IResult<&str, ShellData> {
    map(i32, |v| ShellData::Int(v))(input)
}

fn parse_float(input: &str) -> IResult<&str, ShellData> {
    map(float, |v| ShellData::Float(v))(input)
}

fn parse_number(input: &str) -> IResult<&str, ShellData> {
    parse_float(input)
    //potential to tell if number is actually an integer
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