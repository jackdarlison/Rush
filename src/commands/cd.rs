use std::{vec, path::PathBuf, any::TypeId};

use log::info;

use crate::{architecture::{command::{Command, CommandArgument}, shell_type::ShellType, shell_result::ShellResult, shell_error::ShellError, shell_data::ShellData}, interface::session::Session, helpers::file_system::name, get_type};

#[derive(Debug, Clone)]
pub struct Cd {}

impl Command for Cd {
    get_type!();

    fn name(&self) -> &str {
        "cd"
    }

    fn description(&self) -> &str {
        "Change directory"
    }

    fn options(&self) -> Vec<crate::architecture::command::CommandOption> {
        vec![]
    }

    fn req_arguments(&self) -> Vec<crate::architecture::command::CommandArgument> {
        vec![
            CommandArgument { name: "directory", description: "Directory to move to", arg_type: vec![ShellType::FilePath] }
        ]
    }

    fn list_argument(&self) -> Option<crate::architecture::command::CommandArgument> {
        None
    }

    fn run(&self, mut session: &mut Session, options: Vec<(String, Option<ShellData>)>, arguments: Vec<ShellData>) -> Result<ShellResult, ShellError> {
        if arguments.len() != 1 { return Err(ShellError::InputError(format!("{} takes only 1 argument, {} given", self.name(), arguments.len()))); }

        if let Some(ShellData::FilePath(path)) = arguments.first() {
            let mut dir_name = path.clone();
            if !path.starts_with("/") { dir_name = format!("{}/{}", session.pwd.clone(), path)}
            let path_buf = PathBuf::from(&dir_name);
            if path_buf.is_dir() {
                session.pwd = name(&path_buf);
                info!("Moved to {}", session.pwd.clone())
            } else {
                return Err(ShellError::CommandError(format!("{} is not a directory", &dir_name)));
            }
        } else {
            return Err(ShellError::DataTypeError(format!("Invalid data type given")));
        }

        Ok(ShellResult::None)
    }
}