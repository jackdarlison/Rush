use std::io::stdout;

use crossterm::{terminal::{self, EnableLineWrap, Clear, ClearType, DisableLineWrap}, cursor::{self, SavePosition, RestorePosition}, style::{PrintStyledContent, Stylize, Color}};
use nom::{character::complete::multispace0, sequence::pair};

use crate::parser::commands::{parse_valid_command, parse_options, parse_command};


pub fn scroll_off() {
    if cursor_at_bottom() { 
        execute!(
            stdout(),
            terminal::ScrollUp(2),
            cursor::MoveToPreviousLine(2),
        ).unwrap()
     }
}

pub fn cursor_at_bottom() -> bool {
    let (_, y) = cursor::position().unwrap();
    let (_, y2) = terminal::size().unwrap();
    y2 - y <= 2 //scroll off of two given for consistency
}

pub fn print_hints(input: &str) {
    execute!(
        stdout(),
        EnableLineWrap,
        SavePosition,
        Clear(ClearType::UntilNewLine),
        PrintStyledContent(
            input.with(Color::Cyan)
        ),
        RestorePosition,
        DisableLineWrap,
    ).unwrap()
}

pub fn process_hints(buffer: &String) {
    if let Ok((rest, (command, space))) = pair(parse_valid_command, multispace0)(buffer.as_str()) {
        //only display hints if theres a space after a command
        if !space.starts_with(" ") { return }
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
                        print_hints(format!(" {}", e).as_str())
                    }
                }
                print_hints(format!("{}{}{}", opts, req_args, list_arg).as_str())
            },
            None => {
                print_hints(format!(" unknown command").as_str())
            },
        }
    }
}
