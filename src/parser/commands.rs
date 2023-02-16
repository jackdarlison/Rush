
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
use crate::{architecture::{command::{Command, CommandOption, self}, ast::AstCommand, shell_data::ShellData}, commands::{ls::Ls, echo::Echo}, helpers::lookup::{command_lookup, option_lookup}};

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

    //parse compound options
    let (rest, compound_opts) = opt(tuple((tag("-"), alpha1, multispace0)))(input)?;
    let mut opts: Vec<(&str, Option<&str>)> = vec![];
    match compound_opts {
        Some((_, flags, _)) => {
            let mut comp: Vec<(&str, Option<&str>)> = flags.split("").map(|c| (c, None)).filter(|(c, _)| !c.is_empty()).collect();
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

fn parse_option_helper(command_opts: Vec<CommandOption>) -> impl Fn(&str) -> IResult<&str, (&str, Option<&str>)> {
    move |input| {parse_option(input, command_opts.clone())}
}

fn parse_option(input: &str, command_opts: Vec<CommandOption>) -> IResult<&str, (&str, Option<&str>)> {

    let (rest, (_, opt_name, _)) = tuple((alt((tag("--"), tag("-"))), alpha1, multispace0))(input)?;

    match option_lookup(&command_opts, opt_name) {
        Some(option) => {
            match option.data {
                Some(_) => {
                    let (rest2, (data, _)) = tuple((alpha1, multispace0))(rest)?;
                    Ok((rest2, (opt_name, Some(data))))
                    
                },
                None => Ok((rest, (opt_name, None))),
            }
        },
        None => Err(Failure(Error::new("Not a valid option", ErrorKind::Tag))),
    }
}

fn parse_arguments(input: &str) -> IResult<&str, &str> {
    todo!()
}

fn parse_argument(input: &str) -> IResult<&str, &str> {
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

    #[test]
    fn test_options_parser() {
        println!("{:?}", parse_options("--test hello hello", (Ls {}).options()))
    }

    #[test]
    fn test_option_parser() {
        println!("{:?}", parse_option("--test hello hello", (Ls {}).options()))
    }

}