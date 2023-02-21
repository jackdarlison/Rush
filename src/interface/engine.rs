use std::io::stdout;

use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType, self}, style::{Print, PrintStyledContent, Color, Stylize}, event::{read, Event}, cursor};

use crate::{parser::commands::parse_valid_command, helpers::lookup::command_lookup};

use super::{key_event::process_key_event, session::Session, output::{scroll_off, print_hints}};

pub(crate) fn run() {

   let mut command_buffer = String::new();
   let mut session = Session::new();

   //TODO handle errors?
   enable_raw_mode().unwrap();

    //Initial setup
    execute!(
        stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0,0),
    ).unwrap();

    //start main loop
    'shell_loop: loop {
        //prompt, clear buffer..
        command_buffer.clear();
        execute!(
            stdout(),
            cursor::MoveToNextLine(1),
            PrintStyledContent(format!("{} >> ", session.pwd).with(Color::Green)),
        ).unwrap();

        //start command loop
        'command_loop: loop {

            let side_effects: SideEffects;
            let event = read().unwrap();

            match event {
                Event::Key(ke) => (command_buffer, session, side_effects) = process_key_event(ke, command_buffer, session),
                _ => side_effects = SideEffects::None, //TODO: other events
            }

            //process event
            match side_effects {
                SideEffects::BreakProgram => break 'shell_loop,
                SideEffects::BreakCommand => break 'command_loop,
                SideEffects::ExecuteCommand => {
                    scroll_off();
                    execute!(
                        stdout(),
                        cursor::MoveToNextLine(1),
                        Print(format!("The command was {}", command_buffer)),
                    ).unwrap();
                    //execute Actual command instead
                    break 'command_loop;
                },
                SideEffects::None => {
                    if let Ok((_, command)) = parse_valid_command(command_buffer.as_str()) {
                        match command {
                            Some(c) => {
                                let req_args: String = c.req_arguments().iter().fold(String::new(),|mut acc, arg| {
                                    acc.push_str(" ");
                                    acc.push_str(arg.name);
                                    acc
                                });
                                let mut list_arg = String::new();
                                if let Some(arg) = c.list_argument() {
                                    list_arg.push(' ');
                                    list_arg.push_str(arg.name);
                                    list_arg.push_str("..")
                                }
                                print_hints(format!(" Options{}{}", req_args, list_arg).as_str())
                            },
                            None => {
                                print_hints(format!(" Options Args").as_str())
                            },
                        }
                    } else {
                        print_hints(" Error")
                    }

                    

                    //parse valid command
                    //lookup command
                    //add greyed 
                },
            }
        }
    }

    disable_raw_mode().unwrap();
}

pub enum SideEffects {
    BreakProgram,
    BreakCommand,
    ExecuteCommand,
    None,
}