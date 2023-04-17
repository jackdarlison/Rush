use std::{any::TypeId, path::PathBuf};
use log::info;

use crate::{architecture::{command::{Command, CommandArgument}, shell_type::ShellType, shell_data::ShellData, shell_error::ShellError}, get_type, helpers::file_system::read_file_contents, parser::program::parse_program};


/// Print the abstract syntax tree of a given script file or string
#[derive(Debug, Clone)]
pub struct Ast {}

impl Command for Ast {
    get_type!();

    fn name(&self) -> &str {
        "ast"
    }

    fn description(&self) -> &str {
        "Print the abstract syntax tree of a given script file or string"
    }

    fn options(&self) -> Vec<crate::architecture::command::CommandOption> {
        vec![]
    }

    fn req_arguments(&self) -> Vec<crate::architecture::command::CommandArgument> {
        vec![
            CommandArgument {
                name: "script",
                description: "The file path or string of a script to execute",
                arg_type: vec![
                    ShellType::String,
                    ShellType::FilePath,
                ] 
            },
        ]
    }

    fn list_argument(&self) -> Option<crate::architecture::command::CommandArgument> {
        None
    }

    fn run(&self, session: &mut crate::interface::session::Session, options: Vec<(String, Option<crate::architecture::shell_data::ShellData>)>, arguments: Vec<crate::architecture::shell_data::ShellData>) -> Result<crate::architecture::shell_result::ShellResult, crate::architecture::shell_error::ShellError> {
        if arguments.len() != 1 { return Err(ShellError::InputError(format!("{} takes only 1 argument, {} given", self.name(), arguments.len()))); }

        let script = match arguments.first() {
            Some(ShellData::FilePath(path)) => {
                match read_file_contents(&PathBuf::from(path)) {
                    Ok(script) => script,
                    Err(e) => return Err(ShellError::InputError(format!("Error reading file: {}", e))),
                }
            },
            Some(ShellData::String(script)) => script.to_string(),
            _ => return Err(ShellError::InputError("Invalid data type given".to_string()))
        };

        info!("Parsing for execution: {}", script);

        match parse_program(&script) {
            Ok((rest, ast)) => {
                info!("parse result: {:?} and remaining: {}", ast, rest);
                return Ok(crate::architecture::shell_result::ShellResult::Value(ShellData::String(format!("{:?}", ast))))
            },
            Err(e) => return Err(ShellError::CommandError(format!("Cannot parse command: {}", e))),
        }
    }
}