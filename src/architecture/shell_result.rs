use super::shell_data::ShellData;

#[derive(Debug, Clone)]
pub enum ShellResult {
    Value(ShellData),
    None,
}