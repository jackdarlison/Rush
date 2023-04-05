use std::{any::TypeId, path::PathBuf};
use log::info;

use crate::{get_type, architecture::{command::{Command, CommandArgument}, shell_type::ShellType, shell_result::ShellResult, shell_error::ShellError, shell_data::ShellData}, helpers::file_system::read_file_contents, parser::program::parse_program, interface::{execution::execute_program, formatting::format_shell_results}};


#[derive(Debug, Clone)]
pub struct Rush {}

impl Command for Rush {
    get_type!();

    fn name(&self) -> &str {
        "rush"
    }

    fn description(&self) -> &str {
        "Execute rush scripts"
    }

    fn options(&self) -> Vec<crate::architecture::command::CommandOption> {
        vec![]
    }

    fn req_arguments(&self) -> Vec<crate::architecture::command::CommandArgument> {
        vec![
            CommandArgument {
                name: "script",
                description: "the script to execute",
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

        let parse_result = match parse_program(&script) {
            Ok((rest, ast)) => {
                info!("parse result: {:?} and remaining: {}", ast, rest);
                ast
            },
            Err(e) => return Err(ShellError::CommandError(format!("Cannot parse command: {}", e))),
        };

        let results = match execute_program(parse_result, session) {
            Ok(rs) => format_shell_results(rs),
            Err(e) => return Err(ShellError::CommandError(format!("Cannot execute program: {}", e))),
        };

        info!("Rush command results: {:?}", results);

        match results {
            None => Ok(ShellResult::None),
            Some(r) => Ok(ShellResult::Value(ShellData::String(r)))
        }
    }
}