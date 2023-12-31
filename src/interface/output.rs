use std::{io::stdout, cmp::max};

use crossterm::{terminal::{self, Clear, ClearType}, cursor::{self, SavePosition, RestorePosition, MoveToNextLine, MoveLeft, MoveTo}, style::{PrintStyledContent, Stylize, Color, Print}};

use super::command_buffer::CommandBuffer;

/// Moves the screen up a given ammount
fn scroll_off(n: u16) {
    execute!(
        stdout(),
        terminal::ScrollUp(n),
        cursor::MoveUp(n),
    ).unwrap()
}

/// Returns the distance from the cursor to the bottom of the terminal
fn cursor_to_bottom_distance() -> u16 {
    let (_, y) = cursor::position().unwrap();
    let (_, y2) = terminal::size().unwrap();
    //minimum terminal size is (1,1), minimum cursor position is (0,0) so should never underflow
    y2 - y - 1
}

/// Returns the height of the input text
/// 
/// Respects new line characters
fn get_height_of_text(input: &str) -> u16 {
    let lines = input.split('\n');
    let term_width = terminal::size().unwrap_or((1, 1)).0;
    let mut height: u16 = 0;
    for line in lines {
        height += (line.len().try_into().unwrap_or(1) / term_width) + 1
    }
    height
}

/// Moves the screen up so the cursor is on the top line
pub fn clear() {
    let (_, y) = cursor::position().unwrap();
    scroll_off(y);
}

/// Prints the given prompt
pub fn print_prompt(prompt: &str, colour: Color) {
    if cursor_to_bottom_distance() < 1 {
        scroll_off(1);
    }
    execute!(
        stdout(),
        cursor::MoveToNextLine(1),
        PrintStyledContent(prompt.with(colour)),
    ).unwrap();
}

/// Prints content on the line below the cursor, with the option to restore the posistion of the cursor after printing
pub fn print_below_current(input: &str, restore_pos: bool) {
    if cursor_to_bottom_distance() < get_height_of_text(input) {
        scroll_off(max(0, get_height_of_text(input) - cursor_to_bottom_distance()));
    }
    execute!(
        stdout(),
        SavePosition,
        MoveToNextLine(1),
        Clear(ClearType::FromCursorDown),
        PrintStyledContent(
            input.with(Color::Grey)
        ),
    ).unwrap();
    if restore_pos {
        execute!(stdout(), RestorePosition).unwrap()
    }
}

/// Prints content after the buffer for a given colour
pub fn print_after_input(input: &str, rest_buffer: &str, colour: Color) {
    execute!(
        stdout(),
        SavePosition,
        Clear(ClearType::UntilNewLine),
        Print(rest_buffer),
        Print(" "),
        PrintStyledContent(
            input.with(colour)
        ),
        RestorePosition,
    ).unwrap()
}

/// Refresh the input buffer in the terminal output, and clear after cursor
/// 
/// Resets the cursor posistion to the index within the buffer
pub fn refresh_buffer(prompt_size: u16, buffer: &CommandBuffer) {
    let distance_to_end = buffer.contents.len() - buffer.index;
    execute!(
        stdout(),
        MoveTo(prompt_size, cursor::position().unwrap().1),
        Print(&buffer.contents),
        Clear(ClearType::FromCursorDown),
    ).unwrap();
    if distance_to_end > 0 {
        execute!(
            stdout(),
            MoveLeft(distance_to_end.try_into().unwrap_or(0)),
        ).unwrap();
    }
}


