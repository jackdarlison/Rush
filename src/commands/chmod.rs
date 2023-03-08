
use std::{fs::{Permissions, set_permissions}, path::PathBuf, os::unix::prelude::PermissionsExt};

use log::{info, error};

use crate::architecture::{command::{Command, CommandArgument, CommandOption}, shell_type::ShellType, shell_result::ShellResult, shell_error::ShellError, shell_data::ShellData};


#[derive(Debug)]
pub struct Chmod {}

impl Command for Chmod {
    fn name(&self) -> &str {
        "chmod"
    }

    fn description(&self) -> &str {
        "Change the permissions of a file"
    }

    fn options(&self) -> Vec<crate::architecture::command::CommandOption> {
        vec![]
    }

    fn req_arguments(&self) -> Vec<crate::architecture::command::CommandArgument> {
        vec![
            CommandArgument {
                name:"mode",
                description:"Set the permissions via an absolute octal triplet, in order owner, group, others. Combine read (4), write (2), and execute (1) to get permission number",
                arg_type: vec![ShellType::Octal]
            }
        ]
    }

    fn list_argument(&self) -> Option<crate::architecture::command::CommandArgument> {
        Some(CommandArgument { name: "file_name", description: "File or directory to change permissions of", arg_type: vec![ShellType::FilePath] })
    }

    fn run(&self, session: &mut crate::interface::session::Session, options: Vec<(String, Option<crate::architecture::shell_data::ShellData>)>, arguments: Vec<crate::architecture::shell_data::ShellData>) -> Result<crate::architecture::shell_result::ShellResult, crate::architecture::shell_error::ShellError> {

        if arguments.len() < 2 {
            return Err(ShellError::InputError);
        }

        let mut permissions_code: u32;

        if let Some(ShellData::Int(val)) = arguments.first() {
            if *val > 0o777 { return Err(ShellError::DataTypeError) }
            if let Ok(uval) = (*val).try_into() {
                permissions_code = uval;
            } else {
                return Err(ShellError::DataTypeError);
            }
        } else {
            return Err(ShellError::DataTypeError);
        }

        let files = arguments.split_at(1).1;
        for file in files {
            if let ShellData::FilePath(mut path) = file.clone() {
                if !path.starts_with("/") { path = format!("{}/{}", session.pwd.clone(), &path)}
                let path_buf = PathBuf::from(&path);
                if let Ok(metadata) = path_buf.metadata() {
                    let mut permissions = metadata.permissions();
                    permissions.set_mode(permissions_code);
                    set_permissions(&path, permissions.clone());
                    info!("set {} permissions to {:o}", &path, &permissions.mode());
                } else {
                    return Err(ShellError::CommandError)
                }
            } else {
                return Err(ShellError::DataTypeError);
            }
        }

        Ok(ShellResult::None)
    }
}