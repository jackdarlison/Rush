//! Contains the abstract syntax tree code


use std::ops::Range;

use super::{shell_data::ShellData, command::Command};


/// Contains the root level compound expression
#[derive(Debug, PartialEq)]
pub enum AstProgram {
    Program(Box<AstCompound>)
}

/// Contains the forms of program statements
#[derive(Debug, Clone, PartialEq)]
pub enum AstStatement {
    ControlFlow(AstControlFlow),
    Command(AstCommand),
}

/// Contains the forms of control flow statements
/// 
/// Completed:
/// - [x] For
/// - [ ] If
/// - [ ] While
/// - [ ] Unitl
/// - [ ] Switch
#[derive(Debug, Clone, PartialEq)]
pub enum AstControlFlow {
    For {var: String, range: Range<i32>, body: Box<AstCompound>},
    If,
    While, 
    Until,
    Switch,
}

/// Contains the forms of compound expressions
#[derive(Debug, Clone, PartialEq)]
pub enum AstCompound {
    And(Box<AstCompound>, AstStatement),
    Or(Box<AstCompound>, AstStatement),
    List(Box<AstCompound>, AstStatement),
    Statement(AstStatement),
}

/// Stores a command and the user supplied options and arguments
#[derive(Debug, Clone)]
pub struct AstCommand {
    pub command: Box<dyn Command>,
    pub options: Vec<(String, Option<ShellData>)>,
    pub arguments: Vec<ShellData>,
}

impl PartialEq for AstCommand {
    fn eq(&self, other: &Self) -> bool {
        self.command.get_type() == other.command.get_type() && self.options == other.options && self.arguments == other.arguments
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct AstUnknown {
    pub name: &'static str,
    pub params: Vec<&'static str>,
}

