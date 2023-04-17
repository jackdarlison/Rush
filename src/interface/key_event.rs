use crossterm::event::{KeyEvent, KeyModifiers, KeyEventState, KeyCode};

use super::{session::Session, engine::SideEffects, command_buffer::CommandBuffer};

/// Returns the [`SideEffects`] of a given key event
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
            code: KeyCode::Char(' '),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            side_effects = SideEffects::DisplayCommands;
        },
        KeyEvent {
            code: KeyCode::Char('d'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            side_effects = SideEffects::DisplayDescription;
        },
        KeyEvent {
            code: KeyCode::Char('o'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            side_effects = SideEffects::DisplayOptions;
        },
        KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            side_effects = SideEffects::DisplayArguments;
        },
        KeyEvent {
            code: KeyCode::Char('l'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            side_effects = SideEffects::Clear;
        }
        KeyEvent {
            code: KeyCode::Left,
            .. 
        } => {
            side_effects = SideEffects::MoveLeft;
        },
        KeyEvent { 
            code: KeyCode::Right,
            .. 
        } => {
            side_effects = SideEffects::MoveRight;
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
            side_effects = SideEffects::Delete;
        },
        KeyEvent { 
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::NONE,
            ..
        } => {
            if c.is_ascii() {
                side_effects = SideEffects::Char(c);
            }
        },
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::SHIFT,
            state: KeyEventState::NONE, 
            ..
        } => {
            if c.is_ascii() {
                side_effects = SideEffects::Char(c.to_ascii_uppercase());
            }
        },
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::CAPS_LOCK,
            ..
        } => {
            if c.is_ascii() {
                side_effects = SideEffects::Char(c.to_ascii_uppercase());
            }
        }
        KeyEvent { .. } => (),
    }
    //Return session back to engine
    (buffer, session, side_effects)
}


