use std::vec;

use crate::{architecture::{command::{Command, CommandArgument}, shell_type::ShellType, shell_result::ShellResult, shell_error::ShellError, ast::AstCommand, shell_data::ShellData}, interface::session::Session};



#[derive(Debug, Clone)]
pub struct Cd {}

impl Command for Cd {
    fn name(&self) -> &str {
        "cd"
    }

    fn description(&self) -> &str {
        "Change directory"
    }

    fn options(&self) -> Vec<crate::architecture::command::CommandOption> {
        vec![]
    }

    fn req_arguments(&self) -> Vec<crate::architecture::command::CommandArgument> {
        vec![
            CommandArgument { name: "directory", description: "Directory to move to", arg_type: vec![ShellType::FilePath] }
        ]
    }

    fn list_argument(&self) -> Option<crate::architecture::command::CommandArgument> {
        None
    }

    fn run(&self, session: &Session, options: Vec<(String, Option<ShellData>)>, arguments: Vec<ShellData>) -> Result<ShellResult, ShellError> {
        

        Ok(ShellResult::None)
    }
}