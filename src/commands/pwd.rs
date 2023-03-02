use crate::{architecture::{command::Command, shell_data::ShellData, shell_result::ShellResult, shell_error::ShellError, ast::AstCommand}, interface::session::Session};


#[derive(Debug, Clone)]
pub struct Pwd {}

impl Command for Pwd {
    fn name(&self) -> &str {
        "pwd"
    }

    fn description(&self) -> &str {
        "Return the present working directory"
    }

    fn options(&self) -> Vec<crate::architecture::command::CommandOption> {
        vec![]
    }

    fn req_arguments(&self) -> Vec<crate::architecture::command::CommandArgument> {
        vec![]
    }

    fn list_argument(&self) -> Option<crate::architecture::command::CommandArgument> {
        None
    }

    fn run(&self, session: Session, params: AstCommand) -> Result<ShellResult, ShellError> {
        Ok(ShellResult::Value(ShellData::FilePath(session.pwd)))
    }
}