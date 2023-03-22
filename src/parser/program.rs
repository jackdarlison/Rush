
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