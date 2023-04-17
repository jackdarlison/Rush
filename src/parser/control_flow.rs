use nom::{IResult, character::complete::{alpha1, multispace1, multispace0, space0, space1}, sequence::tuple, bytes::complete::tag, branch::alt};

use crate::{parser::{primitives::parse_range, compound::parse_compound}, architecture::ast::AstControlFlow};

use super::{parser_error::ParserError};

/// Parses any control flow statement
/// 
/// Completed:
/// - [x] for
/// - [ ] if
/// - [ ] while
/// - [ ] until
/// - [ ] switch
pub fn parse_control_flow(input: &str) -> IResult<&str, AstControlFlow, ParserError<&str>> {
    alt((
        parse_for,
        // parse_if,
        // parse_while,
        // parse_until,
        // parse_switch,
    ))(input)
}

/// Parses a for statement
/// 
/// Follows the pattern "for <variable> in <range> { <compound_statement> }"
pub fn parse_for(input: &str) -> IResult<&str, AstControlFlow, ParserError<&str>> {
    let (rest, (_, _, var, _, _, _, range, _)) = tuple((tag("for"), space1, alpha1, space1, tag("in"), space1, parse_range, multispace1))(input)?;
    let (rest, (_, _, body, _, _, _)) = tuple((tag("{"), multispace0, parse_compound, multispace0, tag("}"), space0))(rest)?;
    Ok((rest, AstControlFlow::For { var: var.to_string(), range, body: Box::new(body) }))
}

/// Parses an if statement
pub fn parse_if(input: &str) -> IResult<&str, AstControlFlow, ParserError<&str>> {
    todo!()
}

/// Parses a while statement
pub fn parse_while(input: &str) -> IResult<&str, AstControlFlow, ParserError<&str>> {
    todo!()
}

/// parses an until statement
pub fn parse_until(input: &str) -> IResult<&str, AstControlFlow, ParserError<&str>> {
    todo!()
}

/// Parses a switch statement
pub fn parse_switch(input: &str) -> IResult<&str, AstControlFlow, ParserError<&str>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use crate::{architecture::ast::{AstControlFlow, AstCommand, AstStatement, AstCompound}, commands::{pwd::Pwd}, parser::control_flow::parse_control_flow};


    #[test]
    fn test_for() {
        let output = AstControlFlow::For { var: String::from("i"), range: Range {start: 0, end: 5}, body: Box::new(AstCompound::Statement(AstStatement::Command(AstCommand {command: Box::new(Pwd {}), options: vec![], arguments: vec![]})))};
        assert_eq!(parse_control_flow("for i in 0..<5 {\n    pwd\n}"), Ok(("", output)))
    }

    //TODO: other control flow once implemented
}





