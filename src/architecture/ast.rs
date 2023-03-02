use super::{shell_data::ShellData, command::Command};


#[derive(Debug)]
pub struct AstProgram {

    command: AstCommand,

}

#[derive(Debug)]
pub struct AstCommand {
    pub command: Box<dyn Command>,
    pub options: Vec<(String, Option<ShellData>)>,
    pub arguments: Vec<ShellData>,
}

#[derive(Debug)]
pub struct AstUnknown {
    pub name: &'static str,
    pub params: Vec<&'static str>,
}

