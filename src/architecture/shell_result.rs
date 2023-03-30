use super::shell_data::ShellData;

#[derive(Debug, Clone)]
pub enum ShellResult {
    List(Vec<ShellData>),
    Value(ShellData),
    None,
}
