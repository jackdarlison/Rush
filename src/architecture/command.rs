use crate::architecture::shell_type::ShellType;
use crate::architecture::shell_error::ShellError;
use crate::architecture::shell_result::ShellResult;

//trait for built in commands
pub trait Command {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    //list of possible options for command
    fn options(&self) -> Vec<CommandOption>;
    //ordered list of required arguments
    fn req_arguments(&self) -> Vec<Argument>;
    //list of any non-required commands
    fn other_arguments(&self) -> Option<Vec<Argument>>;
    //optional field for commands with subcommands (like cargo)
    fn subcommand(&self) -> Option<Vec<Box<dyn Command>>>;
    fn run(&self, params: Params) -> Result<ShellResult, ShellError>;
}

pub struct Params {
    pub options: Vec<(&'static str, Option<&'static str>)>,
    pub required_arguments: Vec<(&'static str, &'static str)>,
    pub other_arguments: Vec<&'static str>,
}

impl Params {

    fn add_option(&mut self, name: &'static str, data: Option<&'static str>) {
        self.options.push((name, data));
    }

    fn add_req_arg(&mut self, name: &'static str, data: &'static str) {
        self.required_arguments.push((name, data));
    }

    fn add_other_arg(&mut self, data: &'static str) {
        self.other_arguments.push(data);
    }
}

pub struct Argument {
    name: &'static str,
    description: &'static str,
    //Valid arg types (union)
    arg_type: Vec<ShellType>,
}

pub struct CommandOption {
    name: &'static str,
    short_name: Option<char>,
    description: &'static str,
    data: Option<String>,
    required: bool,
}

