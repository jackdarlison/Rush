
use std::collections::{HashSet, HashMap};

use home;

use crate::architecture::{shell_data::ShellData, shell_result::ShellResult, shell_error::ShellError};

//holds information for the specific user session
#[derive(Debug, Clone)]
pub struct Session {
    pub pwd: String,
    pub vars: HashMap<String, String>,
    pub last_result: Result<ShellResult, ShellError>,
}

impl Session {
    pub fn new() -> Session {
        Session {
            pwd: {
                match home::home_dir() {
                    Some(p) => p.display().to_string(),
                    None => String::from("/"),
                }
            },
            vars: HashMap::new(),
            last_result: Ok(ShellResult::None),
        }
    }

    pub fn set_last_result(&mut self, r: Result<ShellResult, ShellError>) {
        self.last_result = r;
    }
}