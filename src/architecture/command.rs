use crate::architecture::shell_type::ShellType;
use crate::architecture::shell_error::ShellError;
use crate::architecture::shell_result::ShellResult;
use crate::interface::session::Session;
use super::ast::AstCommand;
use super::shell_data::ShellData;
use core::fmt::Debug;

//trait for built in commands
pub trait Command: Debug {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    //list of possible options for command
    fn options(&self) -> Vec<CommandOption>;
    //ordered list of required arguments
    fn req_arguments(&self) -> Vec<CommandArgument>;
    //single argument which can be passed multiple times
    fn list_argument(&self) -> Option<CommandArgument>;

    fn run(&self, session: &mut Session, options: Vec<(String, Option<ShellData>)>, arguments: Vec<ShellData>) -> Result<ShellResult, ShellError>;

}

#[derive(Debug, Clone)]
pub struct CommandArgument {
    pub name: &'static str,
    pub description: &'static str,
    //Valid arg types (union)
    pub arg_type: Vec<ShellType>,
}

#[derive(Debug, Clone, Copy)]
pub struct CommandOption {
    pub name: &'static str,
    pub short_name: Option<&'static str>,
    pub description: &'static str,
    pub data: Option<ShellType>,
    pub required: bool,
}


