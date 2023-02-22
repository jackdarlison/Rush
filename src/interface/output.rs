use std::{io::stdout, fmt::Display, cmp::max};

use crossterm::{terminal::{self, EnableLineWrap, Clear, ClearType, DisableLineWrap}, cursor::{self, SavePosition, RestorePosition, MoveToNextLine}, style::{PrintStyledContent, Stylize, Color}};
use nom::{character::complete::multispace0, sequence::pair};

use crate::{parser::commands::{parse_valid_command, parse_options, parse_command}, architecture::command::Command};


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
    y2 - y - 1
}

pub fn get_height_of_text(input: &str) -> u16 {
    let term_width = terminal::size().unwrap().0;
    let text_width: u16 = input.len().try_into().unwrap();
    text_width / term_width
}

pub fn print_below_current(input: &str) {
    if cursor_to_bottom_distance() < get_height_of_text(input) {
     scroll_off(max(0, get_height_of_text(input) - cursor_to_bottom_distance() + 1));
    }
    execute!(
        stdout(),
        EnableLineWrap,
        SavePosition,
        MoveToNextLine(1),
        Clear(ClearType::FromCursorDown),
        PrintStyledContent(
            input.with(Color::Grey)
        ),
        RestorePosition,
        DisableLineWrap,
    ).unwrap()
}

pub fn print_after_input(input: &str) {
    execute!(
        stdout(),
        EnableLineWrap,
        SavePosition,
        Clear(ClearType::UntilNewLine),
        PrintStyledContent(
            input.with(Color::Cyan)
        ),
        RestorePosition,
        DisableLineWrap,
    ).unwrap()
}


