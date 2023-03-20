use std::{vec, os::unix::fs::symlink, fs::hard_link};
use std::any::TypeId;
use log::{info, error};

use crate::architecture::{command::{Command, CommandOption, CommandArgument}, shell_type::ShellType, shell_error::ShellError, shell_result::ShellResult, shell_data::ShellData};
use crate::get_type;

#[derive(Debug, Clone)]
pub struct Ln {}

impl Command for Ln {
    get_type!();

    fn name(&self) -> &str {
        "ln"
    }

    fn description(&self) -> &str {
        "Create hard (default) or symbolic links between files"
    }

    fn options(&self) -> Vec<crate::architecture::command::CommandOption> {
        vec![
            CommandOption {
                name: "symbolic",
                short_name: Some("s"),
                description: "Create a symbolic link",
                data: None,
                required: false,
            }
        ]
    }

    fn req_arguments(&self) -> Vec<crate::architecture::command::CommandArgument> {
        vec![
            CommandArgument {
                name: "source_file",
                description: "Path of the source file",
                arg_type: vec![ShellType::FilePath]
            },
            CommandArgument {
                name: "link_file",
                description: "Path of link to create",
                arg_type: vec![ShellType::FilePath],
            }
        ]
    }

    fn list_argument(&self) -> Option<crate::architecture::command::CommandArgument> {
        None
    }

    fn run(&self, session: &mut crate::interface::session::Session, options: Vec<(String, Option<crate::architecture::shell_data::ShellData>)>, arguments: Vec<crate::architecture::shell_data::ShellData>) -> Result<crate::architecture::shell_result::ShellResult, crate::architecture::shell_error::ShellError> {
        let is_sym = options.iter().any(|(n, _)| *n=="symbolic");

        if arguments.len() != 2 {
            return Err(ShellError::InputError(format!("{} expects 2 arguments, {} given", self.name(), arguments.len())))
        }

        if let (ShellData::FilePath(mut s), ShellData::FilePath(mut l)) = (arguments[0].clone(), arguments[1].clone()) {
            if !s.starts_with("/") { s = format!("{}/{}", &session.pwd, &s)}
            if !l.starts_with("/") { l = format!("{}/{}", &session.pwd, &l)}
            if is_sym {
                if let Err(e) = symlink(&s, &l) {
                    error!("{:?}", e);
                    return Err(ShellError::CommandError(format!("Error in creating symlink: {}", e)))
                } else {
                    info!("Created symbolic link {} to {}", &l, &s);
                }
            } else {
                if let Err(e) = hard_link(&s, &l) {
                    error!("{:?}", e);
                    return Err(ShellError::CommandError(format!("Error in creating hard link: {}", e)))
                } else {
                    info!("Created hard link {} to {}", &l, &s);
                }
            }
        } else {
            return Err(ShellError::DataTypeError(format!("{} expects file path type arguments", self.name())));
        }


        Ok(ShellResult::None)
    }
}