use std::io::stdout;

use crossterm::{terminal::{Clear, ClearType}, style::Color};
use log::error;
use nom::{sequence::pair, character::complete::{multispace0, space0}};

use crate::{parser::commands::{parse_command, parse_valid_command, parse_options, parse_arguments}, architecture::{shell_result::ShellResult, shell_error::ShellError, command::Command}, helpers::{commands::{format_argument_names}, parser::inner_nom_err}};

pub fn format_hints(buffer: &String) -> (String, Color) {
    if let Ok((rest, (command, _))) = pair(parse_valid_command, space0)(buffer.as_str()) {
        match command {
            Ok(c) => {
                return command_hints(c, rest)
            },
            Err(_e) => {
                return control_flow_hints(buffer);
            },
        }
    }
    (String::new(), Color::Red)
}

fn command_hints(c: Box<dyn Command>, rest: &str) -> (String, Color) {
    let mut s_opts = String::new();
    let mut s_req_args = String::new();
    let mut s_list_arg = String::new();
    if rest.len() == 0 {
        if c.options().len() != 0 {
            s_opts = format!("options.. ");
        }
        s_req_args = format_argument_names(&c);
        if let Some(list_arg) = c.list_argument() {
            s_list_arg = format!(" {}..", list_arg.name);
        }
    } else {
        match parse_options(rest, c.clone()) {
            Ok((o_rest, opts)) => {
                if o_rest.len() == 0 || o_rest.split_whitespace().all(|s| s.starts_with("-")) {
                    s_req_args = format_argument_names(&c);
                    if let Some(list_arg) = c.list_argument() {
                        s_list_arg = format!(" {}..", list_arg.name);
                    }
                } else {
                    match parse_arguments(o_rest, c.clone()) {
                        Ok((a_rest, args)) => {
                            // if leftover from parsing then command is finished
                            if a_rest.len() != 0 {
                                return (String::new(), Color::Red)
                            }
                            // if no args present, display required arg names
                            if args.len() == 0  {
                                s_req_args = format_argument_names(&c);
                                // if no options present display options hint
                                if opts.len() == 0 && c.options().len() != 0 {
                                    s_opts.push_str("options.. ")
                                }
                            } 
                            //display list argument name if present
                            if let Some(list_arg) = c.list_argument() {
                                s_list_arg.push_str(list_arg.name);
                                s_list_arg.push_str("..");
                            }
                        },
                        Err(e) => {
                            return (format!("{}", inner_nom_err(e)), Color::Red);
                        },
                    }
                }
            },
            Err(e) => {
                return (format!("{}", inner_nom_err(e)), Color::Red)
            }
        }
    }
    return (format!("{}{}{}", s_opts, s_req_args, s_list_arg), Color::Cyan)
}

fn control_flow_hints(buffer: &String) -> (String, Color) {
    for_hints(buffer)
}

fn for_hints(buffer: &String) -> (String, Color) {
    let mut words = buffer.split_whitespace();
    match words.next() {
        Some(s) if s == "for" => return (String::from(" variable in range { .. }"), Color::Cyan),
        Some(s) => {
            if words.next() == Option::None && buffer.ends_with(" ") {
                (format!("{} is not a known command", s), Color::Cyan)
            } else {
                (String::new(), Color::Red)
            }
        },
        None => (String::new(), Color::Red),
    }
}

pub fn format_description(buffer: &String) -> String {
    if let Ok((_, Ok(c))) = parse_valid_command(&buffer) {
        return String::from(c.description());
    }
    String::new()
}

pub fn format_options(buffer: &String) -> String {
    if let Ok((_, Ok(c))) = parse_valid_command(&buffer) {
        let options = c.options();
        let mut options_string = options.iter().fold(String::new(), |mut acc, opt| {
            acc.push_str("(");
            if opt.required {
                acc.push_str("R");
            } else {
                acc.push_str("O");
            }
            acc.push_str(") ");
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
        if options_string.len() == 0 {
            options_string = format!("There are no options for {}", c.name());
        }
        return options_string
    }
    String::new()
}

pub fn format_arguments(buffer: &String) -> String {
    if let Ok((_, Ok(c))) = parse_valid_command(&buffer) {
        let mut args = String::new();

        if c.req_arguments().len() != 0 {
            args.push_str("Required Arguments: \r\n");
        }
        
        args.push_str(&c.req_arguments().iter().fold(String::new(), |mut acc, arg| {
            acc.push_str(format!("  {} - {}\r\n", arg.name, arg.description).as_str());
            acc
        }));

        if let Some(arg) = c.list_argument() {
            if args.len() != 0 {
                args.push_str("\r\n");
            }
            args.push_str(format!("List Capable Argument:\r\n").as_str());
            args.push_str(format!("  {} - {}", arg.name, arg.description).as_str());
        }

        if args.len() == 0 {
            args = format!("There are no arguments for {}", c.name());
        }

        return args;
    }
    String::new()
}

    pub fn format_shell_results(results: Vec<ShellResult>) -> Option<String> {
    if results.is_empty() { return None }
    let mut output = String::new();
    for r in results {
        match format_shell_result(r) {
            Some(s) => {
                output.push_str(&s);
                output.push_str("\r\n");
            },
            None => (),
        }
    }
    if output.is_empty() { return None;}
    output.pop();
    output.pop();
    Some(output)
}

pub fn format_shell_result(result: ShellResult) -> Option<String> {
    match result {
        ShellResult::None => None,
        ShellResult::Value(v) => Some(format!("{}", v)),
    }
}
