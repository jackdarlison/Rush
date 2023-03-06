use std::io::stdout;

use crossterm::{event::{KeyEvent, KeyModifiers, KeyEventState, KeyCode}, style::Print, cursor::{MoveLeft, MoveRight, SavePosition, RestorePosition}};

use super::{session::Session, engine::{SideEffects}, output::print_below_current, formatting::{format_description, format_options, format_arguments}, command_buffer::CommandBuffer};



pub fn process_key_event(ke: KeyEvent, mut buffer: CommandBuffer, mut session: Session) -> (CommandBuffer, Session, SideEffects) {

    let mut side_effects = SideEffects::None;

    match ke { 
        KeyEvent {
            code: KeyCode::Esc,
            ..
        } => {
            side_effects = SideEffects::BreakProgram;
        },
        KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            state: KeyEventState::NONE,
            ..
        } => {
            side_effects = SideEffects::BreakCommand;
        },
        KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::NONE,
            ..
        } => {
            side_effects = SideEffects::ExecuteCommand;
        },
        KeyEvent {
            code: KeyCode::Char('d'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            print_below_current(format_description(&buffer.contents).as_str(), true)
        },
        KeyEvent {
            code: KeyCode::Char('o'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            print_below_current(format_options(&buffer.contents).as_str(), true)
        },
        KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            print_below_current(format_arguments(&buffer.contents).as_str(), true)
        },
        KeyEvent {
            code: KeyCode::Left,
            .. 
        } => {
            if buffer.move_left() {
                execute!(stdout(), MoveLeft(1)).unwrap()
            }
        },
        KeyEvent { 
            code: KeyCode::Right,
            .. 
        } => {
            if buffer.move_right() {
                execute!(stdout(), MoveRight(1)).unwrap()
            }
        }
        KeyEvent {
            code: KeyCode::Tab,
            ..
        } => {
            side_effects = SideEffects::AutoComplete
        }
        KeyEvent {
            code: KeyCode::Backspace,
            ..
        } => {
            if let Some(_) = buffer.delete() {
                execute!(
                    stdout(),
                    MoveLeft(1),
                    Print(" "),
                    MoveLeft(1),
                ).unwrap();
            }
        },
        KeyEvent { 
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::NONE,
            ..
        } => {
            buffer.insert(c);
            execute!(
                stdout(),
                Print(c),
                SavePosition,
                Print(buffer.str_contents_after_index()),
                RestorePosition,
            ).unwrap();
        },
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::SHIFT,
            state: KeyEventState::NONE, 
            ..
        } => {
            buffer.insert(c.to_ascii_uppercase());
            execute!(
                stdout(),
                Print(c.to_ascii_uppercase()),
                SavePosition,
                Print(buffer.str_contents_after_index()),
                RestorePosition,
            ).unwrap();
        },
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::CAPS_LOCK,
            ..
        } => {
            buffer.insert(c.to_ascii_uppercase());
            execute!(
                stdout(),
                Print(c.to_ascii_uppercase()),
                SavePosition,
                Print(buffer.str_contents_after_index()),
                RestorePosition,
            ).unwrap();
        }
        KeyEvent { .. } => (),
    }

    //Return session back to engine
    (buffer, session, side_effects)
}


