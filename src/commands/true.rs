use std::any::TypeId;

use crate::{architecture::{command::Command, shell_result::ShellResult}, get_type};


/// Always returns an Ok result
#[derive(Debug, Clone)]
pub struct True {}

impl Command for True {
    get_type!();

    fn name(&self) -> &str {
        "true"
    }

    fn description(&self) -> &str {
        "Returns an empty result"
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
        Ok(ShellResult::None)
    }
}