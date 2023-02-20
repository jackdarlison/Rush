
use std::collections::{HashSet, HashMap};

use home;

//holds information for the specific user session
#[derive(Debug, Clone)]
pub struct Session {
    pub pwd: String,
    pub vars: HashMap<String, String>,
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
        }
    }
}