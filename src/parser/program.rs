
use nom::{IResult, branch::alt, combinator::map};

use crate::architecture::ast::{AstStatement, AstProgram};

use super::{commands::parse_command, parser_error::ParserError, control_flow::parse_control_flow, compound::parse_compound};

pub fn parse_program(input: &str) -> IResult<&str, AstProgram, ParserError<&str>> {
   map(parse_compound, |cc| AstProgram::Program(Box::new(cc)))(input)
}

pub fn parse_statement(input: &str) -> IResult<&str, AstStatement, ParserError<&str>> {
   alt((
      map(parse_control_flow, |cf| {AstStatement::ControlFlow(cf)} ),
      map(parse_command, |c| {AstStatement::Command(c)} ),
   ))(input)
}

#[cfg(test)]
mod tests {
    use crate::{architecture::ast::{AstProgram, AstStatement, AstCommand}, parser::program::parse_program, commands::ls::Ls};


   #[test]
   fn test_program() {
      let output = AstProgram::Program(Box::new(crate::architecture::ast::AstCompound::Statement(AstStatement::Command(AstCommand { command: Box::new(Ls {}), options: vec![], arguments: vec![] }))));
      assert_eq!(parse_program("ls"), Ok(("", output)))
   }

}
