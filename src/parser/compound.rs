use nom::{IResult, branch::alt, sequence::tuple, bytes::complete::tag, character::complete::multispace0, multi::fold_many1};

use crate::architecture::ast::AstCompound;

use super::{parser_error::ParserError, commands::parse_command};


pub fn parse_compound(input: &str) -> IResult<&str, AstCompound, ParserError<&str>> {
    let (rest, command) = parse_command(input)?;
    fold_many1(tuple((alt((tag("&&"), tag("||"), tag(";"))), multispace0, parse_command)),
    move || AstCompound::Command(command.clone()),
    | acc, (op, _, c) | {
        if op == "&&" {
            AstCompound::And(Box::new(acc), c)
        } else if op == "||" {
            AstCompound::Or(Box::new(acc), c)
        } else {
            AstCompound::List(Box::new(acc), c)
        }
    }
    )(rest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", parse_compound("ls -al /Users && pwd"))
    }
}
