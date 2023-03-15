use std::fs::create_dir_all;

use log::info;

use crate::architecture::{command::{Command, CommandArgument, CommandOption}, shell_type::ShellType, shell_error::ShellError, shell_result::ShellResult, shell_data::ShellData};


#[derive(Debug, Clone)]
pub struct Mkdir {}

impl Command for Mkdir {
    fn name(&self) -> &str {
        "mkdir"
    }

    fn description(&self) -> &str {
        "Creates a list of directories"
    }

    fn options(&self) -> Vec<crate::architecture::command::CommandOption> {
        vec![
            CommandOption {
                name: "move",
                short_name: Some("m"),
                description: "Move into the last directory specified",
                data: None,
                required: false
            }
        ]
    }

    fn req_arguments(&self) -> Vec<crate::architecture::command::CommandArgument> {
        vec![]
    }

    fn list_argument(&self) -> Option<crate::architecture::command::CommandArgument> {
        Some(CommandArgument {
            name: "directory_name",
            description: "name of the directory to create",
            arg_type: vec![ShellType::FilePath],
        })
    }

    fn run(&self, session: &mut crate::interface::session::Session, options: Vec<(String, Option<crate::architecture::shell_data::ShellData>)>, arguments: Vec<crate::architecture::shell_data::ShellData>) -> Result<crate::architecture::shell_result::ShellResult, crate::architecture::shell_error::ShellError> {
        if arguments.is_empty() {
            return Err(ShellError::InputError(format!("{} expects at least 1 argument, 0 given", self.name())))
        }
        let move_into = options.iter().any(|(o, _)| *o=="move");

        for argument in arguments {
            if let ShellData::FilePath(mut path) = argument {
                if !path.starts_with("/") { path = format!("{}/{}", session.pwd.clone(), path) }
                if let Err(e) = create_dir_all(&path) {
                    return Err(ShellError::CommandError(format!("Error creating directories: {}", e)));
                }
                info!("Created directory: {}", &path);
                if move_into {
                    session.pwd = path.clone();
                    info!("Moved into: {}", &path);
                }
            } else {
                return Err(ShellError::DataTypeError(format!("{} expecting filepath type arguments", self.name())));
            }
        }

        Ok(ShellResult::None)
    }
}
