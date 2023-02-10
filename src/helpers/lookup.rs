use crate::{architecture::command::Command, commands::{echo::Echo, ls::Ls}};

pub fn command_lookup(command: &str) -> Option<Box<dyn Command>> {
    match command {
        "ls" => Some(Box::new(Ls {})),
        "echo" => Some(Box::new(Echo {})),
        _ => None,
    }
}