use crate::{architecture::{command::*, shell_type::ShellType, shell_result::ShellResult, shell_error::ShellError, shell_data::ShellData, ast::AstCommand}, interface::session::{Session, self}, helpers::file_system::{hidden, name}};

extern crate glob;
use glob::{glob, MatchOptions, glob_with};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Ls {}

impl Command for Ls {

    fn name(&self) -> &str {
        "ls"
    }

    fn description(&self) -> &str {
        "Print the contents of a list of directories"
    }

    fn options(&self) -> Vec<CommandOption> {
        vec![
            CommandOption {
                name: "all",
                short_name: Some("a"),
                description: "Show hidden files",
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
                name: "case_sensitive",
                short_name: Some("c"),
                description: "Make file patterns case sensitive",
                data: None,
                required: false,
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
            name: "directory_name",
            description: "Name of directory",
            arg_type: vec![
                ShellType::FilePath,
            ],       
        })
    }

    fn run(&self, session: &mut Session, options: Vec<(String, Option<ShellData>)>, mut arguments: Vec<ShellData>) -> Result<ShellResult, ShellError> {
        let is_all = options.iter().any(|(n, _)| *n=="all");
        let is_long = options.iter().any(|(n, _)| *n=="long");
        let is_case_sensitive = options.iter().any(|(n, _)| *n=="case_sensitive");

        let match_options = MatchOptions {
            case_sensitive: is_case_sensitive,
            require_literal_separator: false,
            require_literal_leading_dot: !is_all,
        };

        //Run current directory if no arguments
        if arguments.is_empty() {
            arguments.push(ShellData::FilePath(session.pwd.clone()));
        }
        let mut results: Vec<ShellData> = vec![];
        for dir in arguments {
            if let ShellData::FilePath(mut path) = dir {
                path = format!("{}/*", path);
                //if path is local, prepend the current directory
                if !path.starts_with("/") { path = format!("{}/{}", session.pwd, path); }
                for entry in glob_with(&path, match_options).unwrap() {
                    match entry {
                        Ok(path_buf) => {
                            results.push(ShellData::FilePath(name(&path_buf)))
                        },
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
        let res = tester.run(
            &mut Session::new(),
            vec![],
            vec![ShellData::FilePath(String::from("/Users/Jack"))]);
        println!("{:?}", res);
        
        assert!(1==1)
    }

}
