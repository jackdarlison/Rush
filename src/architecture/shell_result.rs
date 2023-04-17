use super::shell_data::ShellData;

/// Contains the different forms results can take
#[derive(Debug, Clone)]
pub enum ShellResult {
    Value(ShellData),
    None,
}