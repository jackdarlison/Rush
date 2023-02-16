

use crate::architecture::{command::*, shell_error::ShellError, shell_type::ShellType, shell_result::ShellResult, params::Params};

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

    fn opt_arguments(&self) -> Vec<CommandArgument> {
        vec![]
    }

    fn list_argument(&self) -> Option<CommandArgument> {
        Some(CommandArgument {
            name: "args",
            description: "The list of arguments to print out",
            arg_type: vec![ShellType::Any],
        })
    }

    fn run(&self, params: Params) -> Result<ShellResult, ShellError> {
        let no_newline = params.options.iter().any(|(n, _)| *n=="no-newline");

        let mut output: String = String::new();

        

        for arg in params.arg_list.into_iter() {
            output.push_str(arg);

            if no_newline {
                output.push(' ');
            } else {
                output.push('\n');
            }
        }


        todo!()
    }

}
