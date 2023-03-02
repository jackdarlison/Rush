use super::shell_data::ShellData;


#[derive(Debug)]
pub struct AstProgram {

    command: AstCommand,

}

#[derive(Debug)]
pub struct AstCommand {
    pub name: String,
    pub options: Vec<(String, Option<ShellData>)>,
    pub arguments: Vec<ShellData>,
}

#[derive(Debug)]
pub struct AstUnknown {
    pub name: &'static str,
    pub params: Vec<&'static str>,
}

