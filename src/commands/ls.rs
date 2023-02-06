use crate::architecture::{command::*, shell_type::ShellType, shell_result::ShellResult, shell_error::ShellError, params::Params};

pub(crate) struct Ls {
    
}

impl Command for Ls {

    fn name(&self) -> &str {
        "ls"
    }

    fn description(&self) -> &str {
        "Print the contents of a directory"
    }

    fn options(&self) -> Vec<CommandOption> {
        vec![
            CommandOption {
                name: "all",
                short_name: Some('a'),
                description: "Print hidden files as well",
                data: None,
                required: false
            },
            CommandOption {
                name: "long",
                short_name: Some('l'),
                description: "Print in long format",
                data: None,
                required: false
            }
        ]
    }

    fn req_arguments(&self) -> Vec<Argument> {
        vec![]
    }

    fn opt_arguments(&self) -> Vec<Argument> {
        vec![]
    }

    fn list_argument(&self) -> Option<Argument> {
        Some(Argument {
            name: "target",
            description: "Path to directory",
            arg_type: vec![
                ShellType::DirPath,
                ShellType::GlobPath,
            ],       
        })
    }

    fn run(&self, params: Params) -> Result<ShellResult, ShellError> {
        let is_all = params.options.iter().any(|(n, _)| *n=="all");
        let is_long = params.options.iter().any(|(n, _)| *n=="long");

        let target_dir = params.req_arguments.iter().find(|a| **a=="target");

        todo!()
    }

}

