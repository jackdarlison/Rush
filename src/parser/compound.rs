use nom::{IResult, branch::alt, sequence::tuple, bytes::complete::tag, character::complete::multispace0, multi::fold_many0};

use crate::architecture::ast::AstCompound;

use super::{parser_error::ParserError, program::parse_statement};

pub fn parse_compound(input: &str) -> IResult<&str, AstCompound, ParserError<&str>> {
    let (rest, command) = parse_statement(input)?;
    fold_many0(tuple((alt((tag("&&"), tag("||"), tag(";"))), multispace0, parse_statement)),
    move || AstCompound::Statement(command.clone()),
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
    use crate::{architecture::ast::{AstStatement, AstCommand}, commands::{ls::Ls, pwd::Pwd}};

    use super::*;

    #[test]
    fn test_compound() {
        let single_output = AstCompound::Statement(AstStatement::Command(AstCommand { command: Box::new(Ls {}), options: vec![], arguments: vec![]}));
        assert_eq!(parse_compound("ls"), Ok(("", single_output)));
        let compound_output_list = AstCompound::List(Box::new(AstCompound::Statement(AstStatement::Command(AstCommand { command: Box::new(Ls {}), options: vec![(String::from("all"), None)], arguments: vec![] }))), AstStatement::Command(AstCommand { command: Box::new(Pwd {}), options: vec![], arguments: vec![]}));
        assert_eq!(parse_compound("ls -a ; pwd"), Ok(("", compound_output_list)));
        let compound_output_or = AstCompound::Or(Box::new(AstCompound::Statement(AstStatement::Command(AstCommand { command: Box::new(Ls {}), options: vec![(String::from("all"), None)], arguments: vec![] }))), AstStatement::Command(AstCommand { command: Box::new(Pwd {}), options: vec![], arguments: vec![]}));
        assert_eq!(parse_compound("ls -a || pwd"), Ok(("", compound_output_or)));
    }
}
