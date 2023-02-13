
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag, 
    combinator::{map, value, opt}, 
    character::complete::{alpha1, alphanumeric1, anychar, multispace1}, 
    multi::{many0, fold_many0}, 
    sequence::{pair, separated_pair}, 
    Err::*,
    error::{Error, ErrorKind},
};
use crate::{architecture::{command::{Command, CommandOption, self}, ast::AstCommand}, commands::{ls::Ls, echo::Echo}, helpers::lookup::{command_lookup}};

fn parse_valid_command(input: &str) -> IResult<&str, &'static str> {
    alt((
        value("ls", tag("ls")),
        value("echo", tag("echo")),
        value("unknown", alpha1),
    ))(input)

    // Ok(("", Box::new(Ls {})))
}

fn parse_command(input: &str) -> IResult<&str, AstCommand> {
    let (rest, name) = parse_valid_command(input)?;
    match command_lookup(name) {
        Some(c) => {
            println!("{:?}", (*c).options()) //WORKS :)
            
            //lookup command information, parse for valid options and arguments
        },
        None => {
            //collect list of arguments.
            //return AstUnknown:w

        },
    }
    todo!() 
}

fn parse_options(input: &str, command_opts: Vec<CommandOption>) -> IResult<&str, Vec<(&str, Option<&str>)>> {

    let (rest, comp) = pair(tag("-"), alpha1)(input)?;

    let opts: Vec<(&str, Option<&str>)> = comp.1.split("").map(|c| (c, None)).collect();

    let (rest, parsed_opts) = fold_many0(
        separated_pair(pair(alt((tag("--"), tag("-"))), alpha1), multispace1, opt(alphanumeric1)),
        move || opts.clone(),
        |mut acc, ((_, name), data) | {
            //this well break on (non data opt, arg). will parse as (data opt, data)
            acc.push((name, data));
            acc
        }
    )(rest)?;

    let valid_short_names: Vec<String> = command_opts.iter().filter_map(|opt| opt.short_name).map(|c| c.to_string()).collect();
    let valid_long_names: Vec<String> = command_opts.iter().map(|opt| opt.name.to_string()).collect();

    // for p_opt in &parsed_opts {
    //     if !(valid_short_names.contains(&String::from(p_opt.0)) || valid_long_names.contains(&String::from(p_opt.0))) {
    //         return Err(Failure(Error::new("Not valid option name", ErrorKind::Tag)))
    //     }

    //     if 
    // } 

    todo!()
}

fn parse_arguments(input: String) -> IResult<String, AstCommand> {
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_command_name() {
        assert_eq!(parse_valid_command("ls"), Ok(("", "ls")))
    }

    #[test]
    fn test_command_parser() {
        parse_command("ls");

        assert!(1 == 1)
    }

}