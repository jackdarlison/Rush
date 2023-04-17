
use std::collections::HashMap;

use home;

use crate::architecture::{shell_result::ShellResult, shell_error::ShellError};

/// Holds information for the specific user session
#[derive(Debug, Clone)]
pub struct Session {
    pub pwd: String,
    pub vars: HashMap<String, String>,
    pub last_result: Result<ShellResult, ShellError>,
}

impl Session {

    /// Creates a new default session
    /// 
    /// Sets the pwd to OS home if possible otherwise root
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

    /// Sets the last result field to a given value
    pub fn set_last_result(&mut self, r: Result<ShellResult, ShellError>) {
        self.last_result = r;
    }
}