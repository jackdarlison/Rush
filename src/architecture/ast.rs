use std::any::TypeId;

use super::{shell_data::ShellData, command::Command};


#[derive(Debug)]
pub struct AstProgram {

    command: AstCommand,

}

#[derive(Debug, Clone)]
pub enum AstCompound {
    And(Box<AstCompound>, AstCommand),
    Or(Box<AstCompound>, AstCommand),
    List(Box<AstCompound>, AstCommand),
    Command(AstCommand),
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

