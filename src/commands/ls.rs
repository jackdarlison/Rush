use std::{path::PathBuf, time::{SystemTime, UNIX_EPOCH}, fs::Metadata};
use std::any::TypeId;
use crate::{architecture::{command::*, shell_type::ShellType, shell_result::ShellResult, shell_error::ShellError, shell_data::ShellData}, interface::session::Session, helpers::file_system::name, get_type};

extern crate chrono;
extern crate glob;
use chrono::NaiveDateTime;
use glob::{MatchOptions, glob_with};

/// Print the contents of a list of directories
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Ls {}

impl Command for Ls {
    get_type!();

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
        let mut results: String = String::new();
        for dir in arguments {
            if let ShellData::FilePath(path) = dir {
                let mut dir_path = path.clone();
                //if path is local, prepend the current directory
                if !dir_path.starts_with("/") { dir_path = format!("{}/{}", session.pwd, path); }
                if !PathBuf::from(&dir_path).is_dir() {
                    return Err(ShellError::InputError(format!("{} is not a directory", &dir_path)))
                }
                dir_path = format!("{}/*", dir_path);
                results.push_str(&format!("/{}:\r\n", path.split("/").last().unwrap()));
                for entry in glob_with(&dir_path, match_options).unwrap() {
                    match entry {
                        Ok(path_buf) => {
                            if is_long {
                                if let Ok(metadata) = path_buf.metadata() {
                                    results.push_str(&format!("{} {} {} {}\r\n", file_type(&metadata), sys_time_to_string(metadata.modified().unwrap()), metadata.len(), name(&path_buf)))
                                } else {
                                    return Err(ShellError::CommandError(format!("Error accessing metadata for {}, maybe the file does not exist?", name(&path_buf))))
                                }
                            } else {
                                results.push_str(&format!("  {}\r\n", name(&path_buf).split("/").last().unwrap()))
                            }
                        },
                        Err(e) => return Err(ShellError::CommandError(format!("Error with file system glob: {}", e))), //TODO: improve error message
                    }
                }
            } else {
                return Err(ShellError::DataTypeError(format!("{} expects file path arguements", self.name())))
            }
        }

        //remove last return + newline
        results.pop();
        results.pop();

        Ok(ShellResult::Value(ShellData::String(results)))

    }

}


fn sys_time_to_string(sys_time: SystemTime) -> String {
    let seconds_since = sys_time.duration_since(UNIX_EPOCH).unwrap().as_secs();
    if let Some(date) = NaiveDateTime::from_timestamp_opt(seconds_since.try_into().unwrap_or(0), 0) {
        format!("{}", date)
    } else {
        String::from("ERROR")
    }
}

fn file_type(metadata: &Metadata) -> String {
    if metadata.is_dir() {
        String::from("Dir")
    } else if metadata.is_file() {
        String::from("File")
    } else {
        String::from("Link")
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


