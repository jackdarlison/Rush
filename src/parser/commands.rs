
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag, 
    combinator::opt, 
    character::complete::{alpha1, space0}, 
    multi::fold_many0, 
    sequence::{pair, tuple}, 
    Err::*,
};
use crate::{architecture::{command::Command, ast::AstCommand, shell_data::ShellData}, helpers::commands::{command_lookup, option_lookup}};

use super::{primitives::{parse_shell_data, parse_shell_data_many}, parser_error::ParserError};

pub fn parse_valid_command(input: &str) -> IResult<&str, Result<Box<dyn Command>, String>, ParserError<&str>> {
    let (rest, command) = alpha1(input)?;
    Ok((rest, command_lookup(command)))
}

pub fn parse_command(input: &str) -> IResult<&str, AstCommand, ParserError<&str>> {
    let (rest, (name, _)) = pair(parse_valid_command, space0)(input)?;
    match name {
        Ok(c) => {
            let (rest, (opts, args)) = pair(
                parse_options_helper(c.clone()),
                parse_arguments_helper(c.clone()),
            )(rest)?;
            Ok((rest, AstCommand {command: c, options: opts, arguments: args}))
        },
        Err(e) => {
            Err(Error(ParserError::CommandError(e)))
            //Collect vec of all arguments and create AstUnknown
        },
    }
}

fn parse_options_helper(command: Box<dyn Command>) -> impl Fn(&str) ->IResult<&str, Vec<(String, Option<ShellData>)>, ParserError<&str>> {
    move |input| {parse_options(input, command.clone())}
}

pub fn parse_options(input: &str, command: Box<dyn Command>) -> IResult<&str, Vec<(String, Option<ShellData>)>, ParserError<&str>> {

    //parse compound options
    let (rest, compound_opts) = opt(tuple((tag("-"), alpha1, space0)))(input)?;
    let mut opts: Vec<(String, Option<ShellData>)> = vec![];
    match compound_opts {
        Some((_, flags, _)) => {
            let short_options: Vec<&str> = flags.split("").filter(|s| !s.is_empty()).collect();
            for so in short_options {
                match option_lookup(command.clone(), so) {
                    Some(o) => {
                        if let None = o.data {
                            opts.push((String::from(o.name), None))
                        } else {
                            return Err(Failure(ParserError::OptionError(format!("{} requires data of type {:?} and cannot be used as a compound option", o.name, o.data.unwrap()))));
                        }
                    },
                    None => {
                        return Err(Failure(ParserError::OptionError(format!("{} is not a valid option for {}", so, command.name()))))

                    }
                }
            }
        },
        None => {},
    }

    fold_many0(
        parse_option_helper(command),
        move || opts.clone(),
        | mut acc, (name, data) | {
            acc.push((name, data));
            acc
        }
    )(rest)

}

fn parse_option_helper(command: Box<dyn Command>) -> impl FnMut(&str) -> IResult<&str, (String, Option<ShellData>), ParserError<&str>> {
    move |input| {parse_option(input, command.clone())}
}

pub fn parse_option(input: &str, command: Box<dyn Command>) -> IResult<&str, (String, Option<ShellData>), ParserError<&str>> {

    let (rest, (_, opt_name, _)) = tuple((alt((tag("--"), tag("-"))), alpha1, space0))(input)?;

    match option_lookup(command.clone(), opt_name) {
        Some(option) => {
            match option.data {
                Some(data_type) => {
                    let (rest2, (data, _)) = tuple((parse_shell_data(data_type), space0))(rest)?;
                    Ok((rest2, (option.name.to_string(), Some(data))))
                    
                },
                None => Ok((rest, (option.name.to_string(), None))),
            }
        },
        None => {
            Err(Failure(ParserError::OptionError(format!("{} is not a valid option for {}", opt_name, command.name()))))
        },
    }
}

pub fn parse_arguments_helper(command: Box<dyn Command>) -> impl Fn(&str) ->IResult<&str, Vec<ShellData>, ParserError<&str>> {
    move |input| {parse_arguments(input, command.clone())}
} 

pub fn parse_arguments(input: &str, command: Box<dyn Command>) -> IResult<&str, Vec<ShellData>, ParserError<&str>> {
    let required = command.req_arguments();
    let list = command.list_argument();

    let mut rest: &str = input;
    let mut arguments = vec![];

    for arg in &required {
        let (next_rest, (argument, _)) = tuple((parse_shell_data_many(arg.arg_type.clone()), space0))(rest)?;
        arguments.push(argument);
        rest = next_rest;
    }

    match list {
        None => Ok((rest, arguments)),
        Some(arg) => {
            fold_many0(
                tuple((parse_shell_data_many(arg.arg_type), space0)),
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

    use crate::{commands::{ls::Ls, chmod::Chmod}, architecture::shell_type::ShellType};

    use super::*;

    #[test]
    fn test_command_parser() {
        let expteced_ls = Ok(("", AstCommand { command: Box::new(Ls {}), options: vec![("all".to_string(), None)], arguments: vec![ShellData::FilePath("/Users".to_string())] }));
        assert_eq!(parse_command("ls -a /Users"), expteced_ls);
        let expected_e = Err(Error(ParserError::CommandError("invalid is not a known command".to_string())));
        assert_eq!(parse_command("invalid --option arg"), expected_e);
    }

    #[test]
    fn test_options_parser() {
        let expected_ls = Ok(("", vec![("all".to_string(), None), ("long".to_string(), None)]));
        assert_eq!(parse_options("-a --long", Box::new(Ls {})), expected_ls);
        let expected_ls_error = Err(Failure(ParserError::OptionError("invalid is not a valid option for ls".to_string())));
        assert_eq!(parse_options("--invalid", Box::new(Ls {})), expected_ls_error);
        //TODO add test for option data (needs command which takes it)
    }

    #[test]
    fn test_argument_parser() {
        let expected_ls = Ok(("", vec![ShellData::FilePath("/Users".to_string())]));
        assert_eq!(parse_arguments("/Users", Box::new(Ls {})), expected_ls);
        let expected_chmod_error = Err(Error(ParserError::DataError(vec![ShellType::Octal])));
        assert_eq!(parse_arguments("invalid file/path", Box::new(Chmod {})), expected_chmod_error);
    }

}