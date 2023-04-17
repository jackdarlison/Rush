use crate::{architecture::command::{Command, CommandOption}, commands::{echo::Echo, ls::Ls, pwd::Pwd, cd::Cd, mkdir::Mkdir, chmod::Chmod, ln::Ln, r#true::True, r#false::False, rush::Rush, ast::Ast}};

#[doc(hidden)]
#[macro_export]
macro_rules! get_type {
    () => {
        fn get_type(&self) -> std::any::TypeId {
            TypeId::of::<Self>()
        }
    };
}

/// Returns a vector of all command instances currently in the program
pub fn commands() -> Vec<Box<dyn Command>> {
    vec![
        Box::new(Ls {}),
        Box::new(Echo {}),
        Box::new(Pwd {}),
        Box::new(Cd {}),
        Box::new(Mkdir {}),
        Box::new(Chmod {}),
        Box::new(Ln {}),
        Box::new(True {}),
        Box::new(False {}),
        Box::new(Rush {}),
        Box::new(Ast {}),
    ]
}

///Returns an instance of a command for a given name
pub fn command_lookup(command: &str) -> Result<Box<dyn Command>, String> {
    commands().into_iter().find(|c| c.name() == command)
        .ok_or(format!("{} is not a known command", command))
}

/// Returns a string containing the formatted argument names
pub fn format_argument_names(command: &Box<dyn Command>) -> String {
    let mut args = command.req_arguments().iter().fold(String::new(), |mut acc, arg| {
        acc.push_str(arg.name);
        acc.push_str(" ");
        acc
    });
    args.pop();
    args
}

/// Returns a [`CommandOption`] for a valid option name and command 
pub fn option_lookup(command: Box<dyn Command>, option: &str) -> Option<CommandOption> {
    command.options().into_iter().find(|o| o.name == option || o.short_name == Some(option))
}

/// Checks if a short option is valid for a given command
pub fn is_valid_short(command: Box<dyn Command>, short: &str) -> bool {
    command.options().iter().any(|opt| opt.short_name == Some(short))
}