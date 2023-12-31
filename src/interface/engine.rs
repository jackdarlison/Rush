use std::io::stdout;

use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType, EnableLineWrap}, style::{Print, Color}, event::{read, Event}, cursor::{self, MoveLeft, MoveRight, SavePosition, RestorePosition}};
use log::{error, info};

use crate::{parser::program::parse_program, helpers::{completion::complete_command, parser::inner_nom_err}, interface::{execution::execute_program, formatting::format_shell_results}, architecture::shell_error::ShellError};

use super::{key_event::process_key_event, session::Session, output::{print_after_input, print_below_current, refresh_buffer, print_prompt, clear}, formatting::{format_hints, format_description, format_options, format_arguments}, command_buffer::CommandBuffer};

/// Runs the main loop of the interface
pub(crate) fn run() {

   let mut command_buffer = CommandBuffer::new();
   let mut autocomplete_buffer: Vec<String> = vec![];
   let mut autocomplete_index: usize = 0;
   let mut session = Session::new();

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
        let prompt = format!("{} >> ", &session.pwd);
        command_buffer.clear();
        let prompt_colour = match session.last_result {
            Ok(_) => Color::Green,
            Err(_) => Color::Red,
        };
        print_prompt(&prompt.clone(), prompt_colour);

        //start command loop
        'command_loop: loop {

            let side_effects: SideEffects;
            let event = read().unwrap();

            match event {
                Event::Key(ke) => (command_buffer, session, side_effects) = process_key_event(ke, command_buffer, session),
                _ => side_effects = SideEffects::None, //TODO: other events
            }

            // if no longer auto complete, reset auto complete buffer
            if let SideEffects::AutoComplete = side_effects {
                // nothing to do here
            } else {
                autocomplete_buffer.clear();
                autocomplete_index = 0;
            }

            //process event
            match side_effects {
                SideEffects::BreakProgram => {
                    info!("breaking program");
                    break 'shell_loop
                },
                SideEffects::BreakCommand => {
                    info!("Breaking command");
                    break 'command_loop
                },
                SideEffects::ExecuteCommand => {

                    // No output on empty command buffer (wanted by student evaluation)
                    if command_buffer.contents.trim().is_empty() {
                        break 'command_loop
                    }

                    info!("Parsing for execution: {}", command_buffer.str_contents());
                    match parse_program(command_buffer.str_contents()) {
                        Ok((rest, parse_result)) => {
                            info!("Left over from parsing: {}", rest);
                            info!("AST generated: {:?}", parse_result);
                            // execute
                            match execute_program(parse_result, &mut session) {
                                Ok(program_result) => {
                                    info!("Results: {:?}", program_result);
                                    if let Some(program_output) = format_shell_results(program_result) {
                                        print_below_current(&program_output, false)
                                    }
                                },
                                Err(e) => {
                                    if let ShellError::None = e {
                                        error!("Error None");
                                    } else {
                                        error!("Execution error: {}", e);
                                        print_below_current(&format!("Execution error: {}", e), false);
                                    }
                                },
                            }
                        },
                        Err(e) => {
                            let err = inner_nom_err(e);
                            error!("{}", err);
                            print_below_current(&format!("{}", err), false)
                        }
                    }

                    break 'command_loop;
                },
                SideEffects::AutoComplete => {
                    if command_buffer.get_word_index() == 0 {
                        info!("Running autocomplete");
                        if autocomplete_buffer.is_empty() {
                            let word = command_buffer.get_current_word().0;
                            autocomplete_buffer.extend(complete_command(word));
                        }
                        if !autocomplete_buffer.is_empty() {
                            if let Some(new_keyword) = autocomplete_buffer.get(autocomplete_index) {
                                command_buffer.replace_current_word(new_keyword);
                                autocomplete_index = (autocomplete_index + 1) % autocomplete_buffer.len();
                                refresh_buffer(prompt.len().try_into().unwrap_or(0), &command_buffer);
                                print_below_current(&format!("{:?}", autocomplete_buffer), true);
                            }
                        } else {
                            print_below_current("No matching commands", true);
                        }
                    }
                }
                SideEffects::DisplayArguments => {
                    print_below_current(format_arguments(&command_buffer.get_context_and_after()).as_str(), true)
                },
                SideEffects::DisplayOptions => {
                    print_below_current(format_options(&command_buffer.get_context_and_after()).as_str(), true);
                },
                SideEffects::DisplayDescription => {
                    print_below_current(format_description(&command_buffer.get_context_and_after()).as_str(), true);
                },
                SideEffects::DisplayCommands => {
                    print_below_current(&format!("{:?}", complete_command("".to_string())), true);
                },
                SideEffects::Clear => {
                    clear();
                },
                SideEffects::Char(c) => {
                    command_buffer.insert(c);
                    execute!(
                        stdout(),
                        Print(c),
                        SavePosition,
                        Print(command_buffer.str_contents_after_index()),
                        RestorePosition,
                    ).unwrap();
                },
                SideEffects::Delete => {
                    if let Some(_) = command_buffer.delete() {
                        execute!(
                            stdout(),
                            MoveLeft(1),
                            Print(" "),
                            MoveLeft(1),
                        ).unwrap();
                    }
                },
                SideEffects::MoveLeft => {
                    if command_buffer.move_left() {
                        execute!(stdout(), MoveLeft(1)).unwrap()
                    }
                },
                SideEffects::MoveRight => {
                    if command_buffer.move_right() {
                        execute!(stdout(), MoveRight(1)).unwrap()
                    }
                },
                SideEffects::None => {
                    // Nothing to do here
                },
            }

            // display hints from last command in buffer
            let (hints, colour) = format_hints(&command_buffer.get_last_context());
            print_after_input(&hints, command_buffer.str_contents_after_index(), colour);
        }
    }

    disable_raw_mode().unwrap();
}

/// Contains the various side effects that may occur from user input
pub enum SideEffects {
    BreakProgram,
    BreakCommand,
    ExecuteCommand,
    AutoComplete,
    DisplayArguments,
    DisplayOptions,
    DisplayDescription,
    DisplayCommands,
    Clear,
    Char(char),
    Delete,
    MoveLeft,
    MoveRight,
    None,
}



