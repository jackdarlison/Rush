
use std::any::TypeId;
use crate::{architecture::{command::*, shell_error::ShellError, shell_type::ShellType, shell_result::ShellResult, shell_data::ShellData}, interface::session::Session, get_type};

/// Print arguments to standard out
#[derive(Debug, Clone, PartialEq)]
pub struct Echo {}

impl Command for Echo {
    get_type!();

    fn name(&self) -> &str {
        "echo"
    }

    fn description(&self) -> &str {
        "Print arguments to standard out"
    }

    fn options(&self) -> Vec<CommandOption> {
        vec![
                CommandOption {
                    name: "no_newline",
                    short_name: Some("n"),
                    description: "Print without the trailing newline character",
                    data: None,
                    required: false
                }
        ]
    }

    fn req_arguments(&self) -> Vec<CommandArgument> {
        vec![]
    }

    fn list_argument(&self) -> Option<CommandArgument> {
        Some(CommandArgument {
            name: "args",
            description: "The list of arguments to print out",
            arg_type: vec![ShellType::String],
        })
    }

    fn run(&self, session: &mut Session, options: Vec<(String, Option<ShellData>)>, arguments: Vec<ShellData>) -> Result<ShellResult, ShellError> {
        let no_newline = options.iter().any(|(n, _)| *n=="no_newline");

        let mut output: String = String::new();

        for arg in arguments.into_iter() {
            if let ShellData::String(val) = arg {
                output.push_str(&val);
                if no_newline {
                    output.push(' ');
                } else {
                    output.push_str("\r\n");
                }
            } else {
                return Err(ShellError::InputError(format!("Expecting string arguments")))
            }
        }

        if !no_newline {
            //remove extra newlines
            output.pop();
            output.pop();
        }

        Ok(ShellResult::Value(ShellData::String(output)))
    }

}
