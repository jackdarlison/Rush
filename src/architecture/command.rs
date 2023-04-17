use dyn_clone::DynClone;

use crate::architecture::shell_type::ShellType;
use crate::architecture::shell_error::ShellError;
use crate::architecture::shell_result::ShellResult;
use crate::interface::session::Session;
use super::shell_data::ShellData;
use core::fmt::Debug;
use std::any::TypeId;

/// Trait for all commands
pub trait Command: Debug + DynClone {
    /// Returns the name of the command
    fn name(&self) -> &str;
    /// Returns the decription for a command
    fn description(&self) -> &str;
    /// Returns a list of possible options for command
    fn options(&self) -> Vec<CommandOption>;
    /// Returns an ordered list of required arguments
    fn req_arguments(&self) -> Vec<CommandArgument>;
    /// Returns a single argument which can be passed multiple times
    fn list_argument(&self) -> Option<CommandArgument>;
    /// Runs a command within a given session with given options and arguments
    fn run(&self, session: &mut Session, options: Vec<(String, Option<ShellData>)>, arguments: Vec<ShellData>) -> Result<ShellResult, ShellError>;
    #[doc(hidden)]
    fn get_type(&self) -> TypeId;
}
dyn_clone::clone_trait_object!(Command);

/// Defines an argument for a command 
#[derive(Debug, Clone)]
pub struct CommandArgument {
    pub name: &'static str,
    pub description: &'static str,
    //Valid arg types (union)
    pub arg_type: Vec<ShellType>,
}

/// Defines an option for a command 
#[derive(Debug, Clone, Copy)]
pub struct CommandOption {
    pub name: &'static str,
    pub short_name: Option<&'static str>,
    pub description: &'static str,
    pub data: Option<ShellType>,
    pub required: bool,
}


