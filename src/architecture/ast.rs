use super::{shell_data::ShellData, command::Command, params::Params};


pub(crate) struct AstProgram {

    command: AstCommand,

}

pub(crate) struct AstCommand {
    name: &'static str,
    options: Vec<(String, ShellData)>,
    arguments: Vec<ShellData>,
    // params: Params,
}

pub(crate) struct AstUnknown {
    name: &'static str,
    params: Vec<&'static str>,
}

