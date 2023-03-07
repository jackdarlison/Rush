
pub fn complete_command(command: String) -> Vec<&'static str> {
    let commands: Vec<&str> = vec!["ls", "echo", "cd", "pwd"];
    commands.into_iter().filter(|c| c.starts_with(command.as_str())).collect()
}