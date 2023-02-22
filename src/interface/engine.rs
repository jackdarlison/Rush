use std::io::stdout;

use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType, self}, style::{Print, PrintStyledContent, Color, Stylize}, event::{read, Event}, cursor};

use crate::{parser::commands::{parse_valid_command, parse_command}, helpers::lookup::command_lookup};

use super::{key_event::process_key_event, session::Session, output::{scroll_off, print_hints, process_hints}};

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
                        Print(format!("Parsed command: {:?}", parse_command(command_buffer.as_str()))),
                    ).unwrap();
                    //TODO execute actual command instead
                    break 'command_loop;
                },
                SideEffects::None => {
                    process_hints(&command_buffer);
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