use crate::architecture::{command::*, shell_type::ShellType, shell_result::ShellResult, shell_error::ShellError, params::Params};

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

    fn req_arguments(&self) -> Vec<CommandArgument> {
        vec![]
    }

    fn opt_arguments(&self) -> Vec<CommandArgument> {
        vec![]
    }

    fn list_argument(&self) -> Option<CommandArgument> {
        Some(CommandArgument {
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

        let target_dir = params.req_args.iter().find(|a| **a=="target");

        let mut results: Vec<String> = vec![];
        for dir in params.arg_list {
            for entry in glob(&(dir.to_owned() + "/*")).unwrap() {
                match entry {
                    Ok(path) => results.push(String::from(path.to_str().unwrap())),
                    Err(_) => return Err(ShellError::UnknownError), //TODO: improve error message
                }
            }
        }

        println!("{:?}", results);
        //TODO:convert data into list of shell data
        todo!()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ls() {

        let tester = Ls {};
        tester.run(Params {options: vec![], req_args: vec![], opt_args: vec![], arg_list: vec!["/Users/"]});
        
        assert!(1==1)
    }

}
