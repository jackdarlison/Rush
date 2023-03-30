use nom::{IResult, character::complete::{alpha1, multispace1, multispace0}, sequence::tuple, bytes::complete::tag, branch::alt};

use crate::{parser::{primitives::parse_range, compound::parse_compound}, architecture::ast::AstControlFlow};

use super::parser_error::ParserError;

pub fn parse_control_flow(input: &str) -> IResult<&str, AstControlFlow, ParserError<&str>> {
    alt((
        parse_for,
        // parse_if,
        // parse_while,
        // parse_until,
        // parse_switch,
    ))(input)
}
pub fn parse_for(input: &str) -> IResult<&str, AstControlFlow, ParserError<&str>> {
    let (rest, (_, _, var, _, _, _, range, _)) = tuple((tag("for"), multispace1, alpha1, multispace1, tag("in"), multispace1, parse_range, multispace1))(input)?;
    let (rest, (_, _, body, _, _)) = tuple((tag("{"), multispace1, parse_compound, tag("}"), multispace0))(rest)?;
    Ok((rest, AstControlFlow::For { var: var.to_string(), range, body: Box::new(body) }))
}

pub fn parse_if(input: &str) -> IResult<&str, AstControlFlow, ParserError<&str>> {
    todo!()
}

pub fn parse_while(input: &str) -> IResult<&str, AstControlFlow, ParserError<&str>> {
    todo!()
}

pub fn parse_until(input: &str) -> IResult<&str, AstControlFlow, ParserError<&str>> {
    todo!()
}

pub fn parse_switch(input: &str) -> IResult<&str, AstControlFlow, ParserError<&str>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use crate::{architecture::ast::{AstControlFlow, AstCommand}, commands::ls::Ls, parser::control_flow::parse_control_flow};


    #[test]
    fn test_for() {
        let output = AstControlFlow::For { var: String::from("i"), range: Range {start: 0, end: 5}, body: Box::new(crate::architecture::ast::AstCompound::Statement(crate::architecture::ast::AstStatement::Command(AstCommand {command: Box::new(Ls {}), options: vec![], arguments: vec![]})))};
        assert_eq!(parse_control_flow("for i in 0..<5 { ls }"), Ok(("", output)))
    }

    //TODO: other control flow once implemented
}





