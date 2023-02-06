use super::shell_data::ShellData;


pub(crate) struct AstProgram {

    command: AstCommand,

}

pub(crate) struct AstCommand {
    name: String,
    options: Vec<(String, ShellData)>,
    arguments: Vec<ShellData>,
}

