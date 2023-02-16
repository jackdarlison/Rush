use crate::{architecture::{command::{Command, CommandOption}, shell_type::ShellType}, commands::{echo::Echo, ls::Ls}};

pub fn command_lookup(command: &str) -> Option<Box<dyn Command>> {
    match command {
        "ls" => Some(Box::new(Ls {})),
        "echo" => Some(Box::new(Echo {})),
        _ => None,
    }
}

pub fn get_command_options(command: &str) -> Option<Vec<CommandOption>> {
    command_lookup(command).and_then(|c| Some(c.options()))
}

pub fn option_lookup(options: &Vec<CommandOption>, option: &str) -> Option<CommandOption> {
    match options.iter().find(|o| o.name == option || o.short_name == Some(option)) {
        Some(o) => Some(*o),
        None => None,
    }
}

pub fn get_option_required(options: &Vec<CommandOption>, option: &str) -> Option<bool> {
    option_lookup(options, option).and_then(|o| Some(o.required))
}

pub fn get_option_short_name(options: &Vec<CommandOption>, option: &str) -> Option<&'static str> {
    option_lookup(options, option).and_then(|o| o.short_name)
}

pub fn get_option_type(options: &Vec<CommandOption>, option: &str) -> Option<ShellType> {
    option_lookup(options, option).and_then(|o| o.data)
}