use crate::{architecture::{command::{Command, CommandOption, CommandArgument}, shell_type::ShellType}, commands::{echo::Echo, ls::Ls, pwd::Pwd, cd::Cd, mkdir::Mkdir, chmod::Chmod, ln::Ln}};

pub fn commands() -> Vec<Box<dyn Command>> {
    vec![
        Box::new(Ls {}),
        Box::new(Echo {}),
        Box::new(Pwd {}),
        Box::new(Cd {}),
        Box::new(Mkdir {}),
        Box::new(Chmod {}),
        Box::new(Ln {}),
    ]
}

pub fn command_lookup(command: &str) -> Result<Box<dyn Command>, String> {
    commands().into_iter().find(|c| c.name() == command)
        .ok_or(format!("{} is not a known command", command))
}

pub fn option_lookup(command: Box<dyn Command>, option: &str) -> Option<CommandOption> {
    command.options().into_iter().find(|o| o.name == option || o.short_name == Some(option))
}

pub fn is_valid_short(command: Box<dyn Command>, short: &str) -> bool {
    command.options().iter().any(|opt| opt.short_name == Some(short))
}