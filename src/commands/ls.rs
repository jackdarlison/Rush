use crate::architecture::{command::*, shell_type::ShellType, shell_result::ShellResult, shell_error::ShellError, params::Params, shell_data::ShellData};

extern crate glob;
use glob::glob;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Ls {}

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
                short_name: Some("a"),
                description: "Print hidden files as well",
                data: None,
                required: false
            },
            CommandOption {
                name: "long",
                short_name: Some("l"),
                description: "Print in long format",
                data: None,
                required: false
            },
            CommandOption {
                name: "test",
                short_name: None,
                description: "Test data options",
                data: Some(ShellType::Any),
                required: false
            }
        ]
    }

    fn req_arguments(&self) -> Vec<CommandArgument> {
        vec![]
    }

    fn list_argument(&self) -> Option<CommandArgument> {
        Some(CommandArgument {
            name: "target",
            description: "Path to directory",
            arg_type: vec![
                ShellType::FilePath,
            ],       
        })
    }

    fn run(&self, params: Params) -> Result<ShellResult, ShellError> {
        let is_all = params.options.iter().any(|(n, _)| *n=="all");
        let is_long = params.options.iter().any(|(n, _)| *n=="long");

        let target_dir = params.req_args.iter().find(|a| **a=="target");

        let mut results: Vec<ShellData> = vec![];
        for dir in params.arg_list {
            println!("{:?}", &dir);
            for entry in glob(&format!("{}/*", dir.to_owned())).unwrap() {
                println!("{:?}", entry);
                match entry {
                    Ok(path) => results.push(ShellData::FilePath(String::from(path.to_str().unwrap()))),
                    Err(_) => return Err(ShellError::UnknownError), //TODO: improve error message
                }
            }
        }

        //TODO: process options, also need to get file information

        Ok(ShellResult::List(results))

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ls() {

        let tester = Ls {};
        let res = tester.run(Params {options: vec![], req_args: vec![], opt_args: vec![], arg_list: vec!["/ouce"]});
        println!("{:?}", res);
        
        assert!(1==1)
    }

}
