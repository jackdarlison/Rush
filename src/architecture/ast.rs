use std::{any::TypeId, ops::Range};

use super::{shell_data::ShellData, command::Command};


#[derive(Debug)]
pub enum AstProgram {
    Program(Box<AstCompound>)
}

#[derive(Debug, Clone)]
pub enum AstStatement {
    ControlFlow(AstControlFlow),
    Command(AstCommand),
}

#[derive(Debug, Clone)]
pub enum AstControlFlow {
    If,
    For {var: String, range: Range<i32>, body: Box<AstCompound>}
}

#[derive(Debug, Clone)]
pub enum AstCompound {
    And(Box<AstCompound>, AstStatement),
    Or(Box<AstCompound>, AstStatement),
    List(Box<AstCompound>, AstStatement),
    Statement(AstStatement),
}

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

#[derive(Debug)]
pub struct AstUnknown {
    pub name: &'static str,
    pub params: Vec<&'static str>,
}

