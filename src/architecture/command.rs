use crate::architecture::shell_type::ShellType;
use crate::architecture::shell_error::ShellError;
use crate::architecture::shell_result::ShellResult;
use super::params::Params;

//trait for built in commands
pub trait Command {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    //list of possible options for command
    fn options(&self) -> Vec<CommandOption>;
    //ordered list of required arguments
    fn req_arguments(&self) -> Vec<Argument>;
    //list of any non-required arguments
    fn opt_arguments(&self) -> Vec<Argument>;
    //argument which can be passed multiple times
    fn list_argument(&self) -> Option<Argument>;
    //optional field for commands with subcommands (like cargo)
    fn run(&self, params: Params) -> Result<ShellResult, ShellError>;
}

pub struct Argument {
    pub name: &'static str,
    pub description: &'static str,
    //Valid arg types (union)
    pub arg_type: Vec<ShellType>,
}

pub struct CommandOption {
    pub name: &'static str,
    pub short_name: Option<char>,
    pub description: &'static str,
    pub data: Option<String>,
    pub required: bool,
}

