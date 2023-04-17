
use nom::{Err::*, IResult, branch::alt, combinator::map};

use crate::architecture::ast::{AstStatement, AstProgram};

use super::{commands::parse_command, parser_error::ParserError, control_flow::parse_control_flow, compound::parse_compound};

/// Parses a valid program
/// 
/// Programs are the root node of the AST containing a compound command
pub fn parse_program(input: &str) -> IResult<&str, AstProgram, ParserError<&str>> {
   map(parse_compound, |cc| AstProgram::Program(Box::new(cc)))(input)
}

/// Parses a valid statement
/// 
/// Statements are either control flow or commands
pub fn parse_statement(input: &str) -> IResult<&str, AstStatement, ParserError<&str>> {
   alt((
      map(parse_command, |c| {AstStatement::Command(c)} ),
      map(parse_control_flow, |cf| {AstStatement::ControlFlow(cf)} ),
   ))(input).map_err(|_| {
      Failure(ParserError::CommandError(format!("{} is not a valid keyword", input.split_whitespace().collect::<Vec<&str>>().first().unwrap_or(&""))))
   })
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use crate::{architecture::ast::{AstProgram, AstStatement, AstCommand, AstCompound, AstControlFlow}, parser::program::parse_program, commands::{ls::Ls, pwd::Pwd}};


   #[test]
   fn test_program() {
      let output = AstProgram::Program(Box::new(crate::architecture::ast::AstCompound::Statement(AstStatement::Command(AstCommand { command: Box::new(Ls {}), options: vec![], arguments: vec![] }))));
      assert_eq!(parse_program("ls"), Ok(("", output)));
      let full = AstProgram::Program(Box::new(AstCompound::Statement(AstStatement::ControlFlow(AstControlFlow::For { var: String::from("i"), range: Range {start: 0, end: 3}, body: Box::new(AstCompound::Statement(AstStatement::Command(AstCommand { command: Box::new(Pwd {}), options: vec![], arguments: vec![] }))) }))));
      assert_eq!(parse_program("for i in 0..<3 {\n    pwd\n}"), Ok(("", full)));
   }

}
