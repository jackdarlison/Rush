
use std::{fmt::Write, str::FromStr};

use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag, 
    combinator::{map, value, opt, peek}, 
    character::complete::{alpha1, alphanumeric1, anychar, multispace1, multispace0}, 
    multi::{many0, fold_many0}, 
    sequence::{pair, separated_pair, delimited, tuple}, 
    Err::{*, self},
    error::{Error, ErrorKind},
};
use crate::{architecture::{command::{Command, CommandOption, self, CommandArgument}, ast::AstCommand, shell_data::ShellData}, commands::{ls::Ls, echo::Echo}, helpers::lookup::{command_lookup, option_lookup, is_valid_short}};

use super::primitives::{parse_shell_data, parse_shell_data_many};

pub fn parse_valid_command(input: &str) -> IResult<&str, Option<Box<dyn Command>>> {
    let (rest, command) = alpha1(input)?;
    Ok((rest, command_lookup(command)))
}

pub fn parse_command(input: &str) -> IResult<&str, AstCommand> {
    let (rest, (name, _)) = pair(parse_valid_command, multispace0)(input)?;
    match name {
        Some(c) => {
            let (rest, (opts, args)) = pair(
                parse_options_helper(c.options()),
                parse_arguments_helper((c.req_arguments(), c.list_argument())),
            )(rest)?;
            Ok((rest, AstCommand {name: c.name().to_string(), options: opts, arguments: args}))
        },
        None => {
            Err(Failure(Error::new("Command not yet implemented", ErrorKind::Tag)))
            //Collect vec of all arguments and create AstUnknown
        },
    }
}

fn parse_options_helper(command_opts: Vec<CommandOption>) -> impl Fn(&str) ->IResult<&str, Vec<(String, Option<ShellData>)>> {
    move |input| {parse_options(input, command_opts.clone())}
}

pub fn parse_options(input: &str, command_opts: Vec<CommandOption>) -> IResult<&str, Vec<(String, Option<ShellData>)>> {

    //parse compound options
    let (rest, compound_opts) = opt(tuple((tag("-"), alpha1, multispace0)))(input)?;
    let mut opts: Vec<(String, Option<ShellData>)> = vec![];
    match compound_opts {
        Some((_, flags, _)) => {
            let mut comp: Vec<(String, Option<ShellData>)> = flags.split("").map(|c| (String::from(c), None)).filter(|(c, _)| !c.is_empty()).collect();
            //TODO improve to display specific invalid option 
            if comp.iter().any(|(name, _)| !is_valid_short(&command_opts, name)) {
                return Err(Failure(Error::new("Invalid compound option", ErrorKind::Tag))) 
            }
            opts.append(&mut comp)
            
        },
        None => {},
    }

    fold_many0(
        parse_option_helper(command_opts),
        move || opts.clone(),
        | mut acc, (name, data) | {
            acc.push((name, data));
            acc
        }
    )(rest)

}

fn parse_option_helper(command_opts: Vec<CommandOption>) -> impl Fn(&str) -> IResult<&str, (String, Option<ShellData>)> {
    move |input| {parse_option(input, command_opts.clone())}
}

pub fn parse_option(input: &str, command_opts: Vec<CommandOption>) -> IResult<&str, (String, Option<ShellData>)> {

    let (rest, (_, opt_name, _)) = tuple((alt((tag("--"), tag("-"))), alpha1, multispace0))(input)?;

    match option_lookup(&command_opts, opt_name) {
        Some(option) => {
            match option.data {
                Some(data_type) => {
                    let (rest2, (data, _)) = tuple((parse_shell_data(data_type), multispace0))(rest)?;
                    Ok((rest2, (opt_name.to_string(), Some(data))))
                    
                },
                None => Ok((rest, (opt_name.to_string(), None))),
            }
        },
        None => {
            Err(Failure(Error::new(opt_name, ErrorKind::Tag)))
        },
    }
}

pub fn parse_arguments_helper(command_args: (Vec<CommandArgument>, Option<CommandArgument>)) -> impl Fn(&str) ->IResult<&str, Vec<ShellData>> {
    move |input| {parse_arguments(input, command_args.clone())}
} 

fn parse_arguments(input: &str, command_args: (Vec<CommandArgument>, Option<CommandArgument>)) -> IResult<&str, Vec<ShellData>> {
    let required = command_args.0;
    let list = command_args.1;

    let mut rest: &str = input;
    let mut arguments = vec![];

    for arg in &required {
        let (next_rest, (argument, _)) = tuple((parse_shell_data_many(arg.arg_type.clone()), multispace0))(rest)?;
        arguments.push(argument);
        rest = next_rest;
    }

    match list {
        None => Ok((rest, arguments)),
        Some(arg) => {
            fold_many0(
                tuple((parse_shell_data_many(arg.arg_type), multispace0)),
                move || arguments.clone(),
                |mut acc, (argument, _)| {
                    acc.push(argument);
                    acc
                }
            )(rest)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_command_name() {
    }

    #[test]
    fn test_command_parser() {
        println!("{:?}", parse_command("ls -al"));

        assert!(1 == 1)
    }

    #[test]
    fn test_options_parser() {
        println!("{:?}", parse_options("", (Ls {}).options()))
    }

    #[test]
    fn test_argument_parser() {
    }

}