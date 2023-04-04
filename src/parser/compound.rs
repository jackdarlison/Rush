use nom::{IResult, branch::alt, sequence::tuple, bytes::complete::tag, multi::fold_many0, character::complete::{multispace0, newline, crlf}};

use crate::architecture::ast::AstCompound;

use super::{parser_error::ParserError, program::parse_statement};

pub fn parse_compound(input: &str) -> IResult<&str, AstCompound, ParserError<&str>> {
    let (rest, command) = parse_statement(input)?;
    fold_many0(tuple((alt((tag("&&"), tag("||"), tag(";"), tag("\n"), tag("\r\n"))), multispace0, parse_statement)),
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
    use std::{ops::Range};

    use crate::{architecture::{ast::{AstStatement, AstCommand, AstControlFlow}, shell_data::ShellData}, commands::{ls::Ls, pwd::Pwd, echo::Echo}};

    use super::*;

    #[test]
    fn test_compound_parser() {
        let single_output = AstCompound::Statement(AstStatement::Command(AstCommand { command: Box::new(Ls {}), options: vec![], arguments: vec![]}));
        assert_eq!(parse_compound("ls"), Ok(("", single_output)));
        let compound_output_list = AstCompound::List(Box::new(AstCompound::Statement(AstStatement::Command(AstCommand { command: Box::new(Ls {}), options: vec![(String::from("all"), None)], arguments: vec![] }))), AstStatement::Command(AstCommand { command: Box::new(Pwd {}), options: vec![], arguments: vec![]}));
        assert_eq!(parse_compound("ls -a ; pwd"), Ok(("", compound_output_list)));
        let compound_output_list_nl = AstCompound::List(Box::new(AstCompound::Statement(AstStatement::Command(AstCommand { command: Box::new(Ls {}), options: vec![(String::from("all"), None)], arguments: vec![] }))), AstStatement::Command(AstCommand { command: Box::new(Pwd {}), options: vec![], arguments: vec![]}));
        assert_eq!(parse_compound("ls -a \n pwd"), Ok(("", compound_output_list_nl)));
        let compound_output_or = AstCompound::Or(Box::new(AstCompound::Statement(AstStatement::Command(AstCommand { command: Box::new(Ls {}), options: vec![(String::from("all"), None)], arguments: vec![] }))), AstStatement::Command(AstCommand { command: Box::new(Pwd {}), options: vec![], arguments: vec![]}));
        assert_eq!(parse_compound("ls -a || pwd"), Ok(("", compound_output_or)));
    }

    #[test]
    fn test_compound_parser_multiline() {
        let output = AstCompound::List(Box::new(AstCompound::Statement(AstStatement::Command(AstCommand { command: Box::new(Pwd {}), options: vec![], arguments: vec![]}))), AstStatement::ControlFlow(AstControlFlow::For { var: String::from("i"), range: Range {start: 0, end: 3}, body: Box::new(AstCompound::List(Box::new(AstCompound::Statement(AstStatement::Command(AstCommand { command: Box::new(Echo {}), options: vec![], arguments: vec![ShellData::String(String::from("test"))] }))), AstStatement::Command(AstCommand { command: Box::new(Pwd {}), options: vec![], arguments: vec![] })))}));
        assert_eq!(parse_compound("pwd\nfor i in 0..<3 {\n    echo \"test\"\n    pwd\n}"), Ok(("", output)));
    }
}
