use super::shell_data::ShellData;

pub enum ShellResult {

    List(Vec<ShellData>),
    Value(ShellData),
    None,
    
}
