

use crate::{architecture::{command::*, shell_error::ShellError, shell_type::ShellType, shell_result::ShellResult, ast::AstCommand, shell_data::ShellData}, interface::session::Session};

#[derive(Debug, Clone, PartialEq)]
pub struct Echo {}

impl Command for Echo {

    fn name(&self) -> &str {
        "echo"
    }

    fn description(&self) -> &str {
        "arguments to standard out"
    }

    fn options(&self) -> Vec<CommandOption> {
        vec![
                CommandOption {
                    name: "no-newline",
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
            arg_type: vec![ShellType::Any],
        })
    }

    fn run(&self, session: Session, params: AstCommand) -> Result<ShellResult, ShellError> {
        let no_newline = params.options.iter().any(|(n, _)| *n=="no-newline");

        let mut output: String = String::new();

        for arg in params.arguments.into_iter() {
            if let ShellData::String(val) = arg {
                output.push_str(&val);
                if no_newline {
                    output.push(' ');
                } else {
                    output.push('\n');
                }
            } else {
                return Err(ShellError::InputError)
            }
        }

        todo!()
    }

}
