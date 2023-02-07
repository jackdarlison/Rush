
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag, 
    combinator::{map, value}, character::complete::alpha1,
};
use crate::{architecture::{command::Command, ast::AstCommand}, commands::{ls::Ls, echo::Echo}};


fn parse_valid_command(input: &str) -> IResult<&str, &'static str> {
    alt((
        value("ls", tag("ls")),
        value("echo", tag("echo")),
        value("unknown", alpha1),
    ))(input)

    // Ok(("", Box::new(Ls {})))
}

fn parse_command(input: &str) -> IResult<&str, AstCommand> {
    let (rest, name) = parse_valid_command(input)?;
    if name == "unknown" {
        //collect list of arguments.
        //return AstUnknown
    }
    //lookup command information, parse for valid options and arguments

    todo!() 
}

fn parse_options(input: String) -> IResult<String, AstCommand> {
    todo!()
}

fn parse_arguments(input: String) -> IResult<String, AstCommand> {
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_command_name() {
        assert_eq!(parse_valid_command("ls"), Ok(("", "ls")))
    }

}