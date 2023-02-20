use std::io::stdout;

use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType, self}, style::{Print, PrintStyledContent, Color, Stylize}, event::{read, Event}, cursor};

use super::{key_event::process_key_event, session::Session};

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
                    if cursor_at_bottom() { 
                        execute!(
                            stdout(),
                            terminal::ScrollUp(2),
                            cursor::MoveToPreviousLine(2),
                        ).unwrap()
                     }
                    execute!(
                        stdout(),
                        cursor::MoveToNextLine(1),
                        Print(format!("The command was {}", command_buffer)),
                    ).unwrap();
                    //execute Actual command instead
                    break 'command_loop;
                },
                SideEffects::None => (),
            }
        }
    }

    disable_raw_mode().unwrap();
}

fn cursor_at_bottom() -> bool {
    let (_, y) = cursor::position().unwrap();
    let (_, y2) = terminal::size().unwrap();
    y2 - y <= 2 //scroll off of two given for consistency
}

pub enum SideEffects {
    BreakProgram,
    BreakCommand,
    ExecuteCommand,
    None,
}