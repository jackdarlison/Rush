
use nom::IResult;

use crate::{architecture::{command::Command, ast::AstCommand}, commands::{ls::Ls, echo::Echo}};


fn get_command_struct(name: &str) -> Option<Box<dyn Command>> {
    match name {
        "ls" => Some(Box::new(Ls {})),
        "echo" => Some(Box::new(Echo {})),
        _ => None,
    }
}

fn parse_command(input: String) -> IResult<String, AstCommand> {
    todo!()
}

fn parse_options(input: String) -> IResult<String, AstCommand> {
    todo!()
}

fn parse_arguments(input: String) -> IResult<String, AstCommand> {
    todo!()
}

