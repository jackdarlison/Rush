use crate::architecture::shell_type::ShellType;
use crate::architecture::shell_error::ShellError;
use crate::architecture::shell_result::ShellResult;
use super::params::Params;
use core::fmt::Debug;

//trait for built in commands
pub(crate) trait Command: Debug {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    //list of possible options for command
    fn options(&self) -> Vec<CommandOption>;
    //ordered list of required arguments
    fn req_arguments(&self) -> Vec<CommandArgument>;
    //list of any non-required arguments
    fn opt_arguments(&self) -> Vec<CommandArgument>;
    //single argument which can be passed multiple times
    fn list_argument(&self) -> Option<CommandArgument>;

    fn run(&self, params: Params) -> Result<ShellResult, ShellError>;

}

pub(crate) struct CommandArgument {
    pub name: &'static str,
    pub description: &'static str,
    //Valid arg types (union)
    pub arg_type: Vec<ShellType>,
}

pub(crate) struct CommandOption {
    pub name: &'static str,
    pub short_name: Option<char>,
    pub description: &'static str,
    pub data: Option<String>,
    pub required: bool,
}

