use std::io::stdout;

use crossterm::{terminal::{self, EnableLineWrap, Clear, ClearType, DisableLineWrap}, cursor::{self, SavePosition, RestorePosition}, style::{PrintStyledContent, Stylize, Color}};


pub fn scroll_off() {
    if cursor_at_bottom() { 
        execute!(
            stdout(),
            terminal::ScrollUp(2),
            cursor::MoveToPreviousLine(2),
        ).unwrap()
     }
}

pub fn cursor_at_bottom() -> bool {
    let (_, y) = cursor::position().unwrap();
    let (_, y2) = terminal::size().unwrap();
    y2 - y <= 2 //scroll off of two given for consistency
}

pub fn print_hints(input: &str) {
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
