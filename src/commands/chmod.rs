use std::any::TypeId;
use std::{fs::set_permissions, path::PathBuf, os::unix::prelude::PermissionsExt};

use log::{info, error};

use crate::{architecture::{command::{Command, CommandArgument}, shell_type::ShellType, shell_result::ShellResult, shell_error::ShellError, shell_data::ShellData}, get_type};


/// Change the permissions of a list of files via an octal triplet
#[derive(Debug, Clone)]
pub struct Chmod {}

impl Command for Chmod {
    get_type!();

    fn name(&self) -> &str {
        "chmod"
    }

    fn description(&self) -> &str {
        "Change the permissions of a list of files via an octal triplet"
    }

    fn options(&self) -> Vec<crate::architecture::command::CommandOption> {
        vec![]
    }

    fn req_arguments(&self) -> Vec<crate::architecture::command::CommandArgument> {
        vec![
            CommandArgument {
                name:"mode",
                description:"Set the permissions via an absolute octal triplet. Digits are in order owner, group, others, where combining additively read (4), write (2), and execute (1) gives the related permission number",
                arg_type: vec![ShellType::Octal]
            }
        ]
    }

    fn list_argument(&self) -> Option<crate::architecture::command::CommandArgument> {
        Some(CommandArgument { name: "file_name", description: "File or directory to change permissions of", arg_type: vec![ShellType::FilePath] })
    }

    fn run(&self, session: &mut crate::interface::session::Session, options: Vec<(String, Option<crate::architecture::shell_data::ShellData>)>, arguments: Vec<crate::architecture::shell_data::ShellData>) -> Result<crate::architecture::shell_result::ShellResult, crate::architecture::shell_error::ShellError> {

        if arguments.len() < 2 {
            return Err(ShellError::InputError(format!("{} needs at least 2 arguments, {} given", self.name(), arguments.len())));
        }

        let mut permissions_code: u32;

        if let Some(ShellData::Int(val)) = arguments.first() {
            if *val > 0o777 { return Err(ShellError::DataTypeError(format!("Value ({:o}) is over 777", val))) }
            if let Ok(uval) = (*val).try_into() {
                permissions_code = uval;
            } else {
                return Err(ShellError::DataTypeError(format!("{} can not be made into an unsigned integer", val)));
            }
        } else {
            return Err(ShellError::DataTypeError(format!("First argument is expecting an integer")));
        }

        let files = arguments.split_at(1).1;
        for file in files {
            if let ShellData::FilePath(mut path) = file.clone() {
                if !path.starts_with("/") { path = format!("{}/{}", session.pwd.clone(), &path)}
                let path_buf = PathBuf::from(&path);
                if let Ok(metadata) = path_buf.metadata() {
                    let mut permissions = metadata.permissions();
                    permissions.set_mode(permissions_code);
                    if let Err(e) = set_permissions(&path, permissions.clone()) {
                        error!("{:?}", e);
                        return Err(ShellError::CommandError(format!("Error setting permissions for {} - {}", path, e)))
                    } else {
                        info!("set {} permissions to {:o}", &path, &permissions.mode());
                    }
                } else {
                    return Err(ShellError::CommandError(format!("Metadata for {} is inaccessible, maybe the file does not exist?", path)))
                }
            } else {
                return Err(ShellError::DataTypeError(format!("Arguments are expected to be file paths")));
            }
        }

        Ok(ShellResult::None)
    }
}