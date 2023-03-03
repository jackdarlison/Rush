use crate::{architecture::{command::*, shell_type::ShellType, shell_result::ShellResult, shell_error::ShellError, shell_data::ShellData, ast::AstCommand}, interface::session::{Session, self}};

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

    fn run(&self, session: &Session, options: Vec<(String, Option<ShellData>)>, arguments: Vec<ShellData>) -> Result<ShellResult, ShellError> {
        let is_all = options.iter().any(|(n, _)| *n=="all");
        let is_long = options.iter().any(|(n, _)| *n=="long");

        let mut results: Vec<ShellData> = vec![];
        for dir in arguments {
            if let ShellData::FilePath(path) = dir {
                let mut path = format!("{}/*", path);
                if !path.starts_with("/") { path = format!("{}/{}", session.pwd, path); }
                for entry in glob(&path).unwrap() {
                    match entry {
                        Ok(path) => results.push(ShellData::FilePath(String::from(path.to_str().unwrap()))),
                        Err(_) => return Err(ShellError::UnknownError), //TODO: improve error message
                    }
                }
            } else {
                return Err(ShellError::DataTypeError)
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
        let res = tester.run(&Session::new(), vec![], vec![ShellData::FilePath(String::from("/Users/Jack/Documents"))]);
        println!("{:?}", res);
        
        assert!(1==1)
    }

}
