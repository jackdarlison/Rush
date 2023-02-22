use std::io::stdout;

use crossterm::terminal::{Clear, ClearType};
use nom::{sequence::pair, character::complete::multispace0};

use crate::parser::commands::{parse_command, parse_valid_command};

use super::output::print_after_input;

pub fn format_hints(buffer: &String) -> String {
    if let Ok((rest, (command, space))) = pair(parse_valid_command, multispace0)(buffer.as_str()) {
        //only display hints if theres a space after a command
        if !space.starts_with(" ") { 
            execute!(stdout(), Clear(ClearType::UntilNewLine)).unwrap();
            return String::new();
        }
        match command {
            Some(c) => {
                let mut opts = String::new();
                let mut req_args = String::new();
                let mut list_arg = String::new();
                match parse_command(buffer.as_str()) {
                    Ok((_, ast)) => {
                        if ast.arguments.len() == 0 {
                            //If no parsed arguments display args spec
                            req_args = c.req_arguments().iter().fold( req_args,|mut acc, arg| {
                                acc.push_str(" ");
                                acc.push_str(arg.name);
                                acc
                            });
                            if let Some(arg) = c.list_argument() {
                                list_arg.push(' ');
                                list_arg.push_str(arg.name);
                                list_arg.push_str("..")
                            }
                            //if also no options display options
                            if ast.options.len() == 0 {
                                opts.push_str("options");
                            }
                        }
                    },
                    Err(e) => {
                        return format!(" {}", e)
                    }
                }
                return format!("{}{}{}", opts, req_args, list_arg)
            },
            None => {
                return format!(" unknown command")
            },
        }
    }
    String::new()
}

pub fn format_description(buffer: &String) -> String {
    if let Ok((_, Some(c))) = parse_valid_command(&buffer) {
        return String::from(c.description());
    }
    String::new()
}

pub fn format_options(buffer: &String) -> String {
    if let Ok((_, Some(c))) = parse_valid_command(&buffer) {
        let options = c.options();
        let options_string = options.iter().fold(String::new(), |mut acc, opt| {
            acc.push_str("- ");
            acc.push_str(opt.name);
            acc.push_str(" (");
            if let Some(short) = opt.short_name {
                acc.push_str(short);
            } else {
                acc.push_str(" ");
            }
            acc.push_str(") ");
            acc.push_str(opt.description);
            acc.push_str("\r\n");
            acc
        });
        return options_string
    }
    String::new()
}

pub fn format_arguments(buffer: &String) -> String {
    if let Ok((_, Some(c))) = parse_valid_command(&buffer) {
        let mut args = String::new();

        args.push_str("Required Arguments: \r\n");
        
        args.push_str(&c.req_arguments().iter().fold(String::new(), |mut acc, arg| {
            acc.push_str(format!("{} - {}\r\n", arg.name, arg.description).as_str());
            acc
        }));

        args.push_str(format!("----------\r\nList Argument:\r\n").as_str());

        if let Some(arg) = c.list_argument() {
            args.push_str(format!("{} - {}", arg.name, arg.description).as_str());
        }

        return args;
    }
    String::new()
}