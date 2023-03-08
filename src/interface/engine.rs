use std::{io::stdout, cmp};

use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType, EnableLineWrap}, style::{Print, PrintStyledContent, Color, Stylize}, event::{read, Event}, cursor::{self, MoveLeft, MoveRight}};

use crate::{parser::commands::parse_command, helpers::completion::complete_command};

use super::{key_event::process_key_event, session::Session, output::{scroll_off, cursor_to_bottom_distance, print_after_input, print_below_current, refresh_buffer}, formatting::format_hints, command_buffer::CommandBuffer};

pub(crate) fn run() {

   let mut command_buffer = CommandBuffer::new();
   let mut autocomplete_buffer: Vec<&str> = vec![];
   let mut autocomplete_index: usize = 0;
   let mut session = Session::new();
   let mut prompt: String = format!("{} >> ", &session.pwd);

   //TODO handle errors?
   enable_raw_mode().unwrap();

    //Initial setup
    execute!(
        stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0,0),
        EnableLineWrap,
    ).unwrap();

    //start main loop
    'shell_loop: loop {
        //prompt, clear buffer..
        command_buffer.clear();
        execute!(
            stdout(),
            cursor::MoveToNextLine(1),
            PrintStyledContent(prompt.clone().with(Color::Green)),
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

                    let result = parse_command(command_buffer.str_contents());

                    if let Ok((_, ast)) = result {
                        print_below_current(&format!("command output: {:?}", ast.command.run(&session, ast.options, ast.arguments)), false);
                    } else {
                        print_below_current("Error", false)
                    }
                    //TODO execute actual command instead
                    break 'command_loop;
                },
                SideEffects::AutoComplete => {
                    if autocomplete_buffer.is_empty() {
                        let word = command_buffer.get_current_word().0;
                        autocomplete_buffer.extend(complete_command(word));
                    }
                    if !autocomplete_buffer.is_empty() {
                        command_buffer.replace_current_word(autocomplete_buffer[autocomplete_index]);
                        autocomplete_index = (autocomplete_index + 1) % autocomplete_buffer.len();
                        refresh_buffer(prompt.len().try_into().unwrap_or(0), &command_buffer);
                        print_below_current(&format!("{:?}", autocomplete_buffer), true);
                    } else {
                        print_below_current("No matching commands", true);
                    }
                }
                SideEffects::None => {
                    autocomplete_buffer.clear();
                    autocomplete_index = 0;
                    print_after_input(format_hints(&command_buffer.contents).as_str(), command_buffer.str_contents_after_index());
                    print_below_current(&format!("{:?} {:?}", command_buffer, command_buffer.get_current_word()), true);
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
    AutoComplete,
    None,
}



