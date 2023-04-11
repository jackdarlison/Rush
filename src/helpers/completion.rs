
pub fn complete_command(command: String) -> Vec<&'static str> {
    keywords().into_iter().filter(|c| c.starts_with(command.as_str())).collect()
}

pub fn keywords() -> Vec<&'static str> {
    vec![
        // Commands
        "ls", "echo", "cd", "pwd", "mkdir", "chmod", "ln", "false", "true", "rush", "ast",
        // Control flow
        "for",
    ]
}

pub fn seperators() -> Vec<&'static str> {
    vec![
        // Compound commands
        "&&", "||", ";", "\r\n", "\n",
        // Control flow
        "}",
    ]
}