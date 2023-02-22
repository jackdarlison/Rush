use std::io::stdout;

use crossterm::{event::{KeyEvent, KeyModifiers, KeyEventKind, KeyEventState, KeyCode}, style::Print, terminal::{self, Clear}, cursor::MoveLeft};

use super::{session::Session, engine::SideEffects};



pub fn process_key_event(ke: KeyEvent, mut buffer: String, mut session: Session) -> (String, Session, SideEffects) {

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
            code: KeyCode::Backspace,
            ..
        } => {
            if let Some(_) = buffer.pop() {
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
            buffer.push(c);
            execute!(stdout(), Print(c)).unwrap();
        },
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::SHIFT,
            state: KeyEventState::NONE, 
            ..
        } => {
            buffer.push(c.to_ascii_uppercase());
            execute!(stdout(), Print(c.to_ascii_uppercase())).unwrap();
        },
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::CAPS_LOCK,
            ..
        } => {
            buffer.push(c.to_ascii_uppercase());
            execute!(stdout(), Print(c.to_ascii_uppercase())).unwrap();
        }
        KeyEvent { .. } => (),
    }

    //Return session back to engine
    (buffer, session, side_effects)
}


