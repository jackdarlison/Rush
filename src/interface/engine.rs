use std::io::stdout;

use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType}, style::{Print, PrintStyledContent, Color, Stylize}, event::{read, Event}, cursor};

use crate::parser::commands::parse_command;

use super::{key_event::process_key_event, session::Session, output::{scroll_off, cursor_to_bottom_distance, print_after_input}, formatting::format_hints};

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
            PrintStyledContent(format!("{} >> ", &session.pwd).with(Color::Green)),
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
                    if cursor_to_bottom_distance() < 2 { scroll_off(2) }

                    let result = parse_command(&command_buffer.as_str());

                    if let Ok((_, ast)) = result {
                        execute!(
                            stdout(),
                            cursor::MoveToNextLine(1),
                            Clear(ClearType::FromCursorDown),
                            Print(format!("command output: {:?}", ast.command.run(&session, ast.options, ast.arguments))),
                        ).unwrap();
                    } else {
                        execute!(
                            stdout(),
                            cursor::MoveToNextLine(1),
                            Clear(ClearType::FromCursorDown),
                            Print(format!("Error")),
                        ).unwrap(); 
                    }
                    //TODO execute actual command instead
                    break 'command_loop;
                },
                SideEffects::None => {
                    print_after_input(format_hints(&command_buffer).as_str());
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