use std::any::TypeId;
use crate::{architecture::{command::Command, shell_error::ShellError}, get_type};


#[derive(Debug, Clone)]
pub struct False {}

impl Command for False {
    get_type!();

    fn name(&self) -> &str {
        "false"
    }

    fn description(&self) -> &str {
        "Returns an error"
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

    fn run(&self, session: &mut crate::interface::session::Session, options: Vec<(String, Option<crate::architecture::shell_data::ShellData>)>, arguments: Vec<crate::architecture::shell_data::ShellData>) -> Result<crate::architecture::shell_result::ShellResult, crate::architecture::shell_error::ShellError> {
        Err(ShellError::None)
    }


}

