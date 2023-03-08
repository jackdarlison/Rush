use std::{io::stdout, cmp::max};

use crossterm::{terminal::{self, Clear, ClearType}, cursor::{self, SavePosition, RestorePosition, MoveToNextLine, MoveRight, MoveLeft, MoveTo}, style::{PrintStyledContent, Stylize, Color, Print}};

use super::command_buffer::CommandBuffer;


pub fn scroll_off(n: u16) {
    execute!(
        stdout(),
        terminal::ScrollUp(n),
        cursor::MoveToPreviousLine(n),
    ).unwrap()
}

pub fn cursor_to_bottom_distance() -> u16 {
    let (_, y) = cursor::position().unwrap();
    let (_, y2) = terminal::size().unwrap();
    //minimum terminal size is (1,1), minimum cursor position is (0,0) so should never underflow
    y2 - y - 1
}

pub fn get_height_of_text(input: &str) -> u16 {
    let term_width = terminal::size().unwrap_or((1, 1)).0;
    let text_width: u16 = input.len().try_into().unwrap_or(1);
    text_width / term_width
}

pub fn print_below_current(input: &str, restore_pos: bool) {
    if cursor_to_bottom_distance() < get_height_of_text(input) {
        scroll_off(max(0, get_height_of_text(input) - cursor_to_bottom_distance() + 3));
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

pub fn print_after_input(input: &str, rest_buffer: &str) {
    execute!(
        stdout(),
        SavePosition,
        Clear(ClearType::UntilNewLine),
        Print(rest_buffer),
        PrintStyledContent(
            input.with(Color::Cyan)
        ),
        RestorePosition,
    ).unwrap()
}

pub fn refresh_buffer(prompt_size: u16, buffer: &CommandBuffer) {
    let distance_to_end = buffer.contents.len() - buffer.index;
    execute!(
        stdout(),
        MoveTo(prompt_size, cursor::position().unwrap().1),
        Print(&buffer.contents),
        Clear(ClearType::UntilNewLine),
    ).unwrap();
    if distance_to_end > 0 {
        execute!(
            stdout(),
            MoveLeft(distance_to_end.try_into().unwrap_or(0)),
        ).unwrap();
    }
}


