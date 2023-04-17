use crate::helpers::commands::commands;

/// Returns a list of keywords whose start match with the supplied string
pub fn complete_command(command: String) -> Vec<String> {
    keywords().into_iter().filter(|c| c.starts_with(command.as_str())).collect()
}

/// Returns a list of all keywords
/// 
/// Keywords are command names and control flow initators
pub fn keywords() -> Vec<String> {
    let command_names = commands().into_iter().map(|c| c.name().to_owned());
    
    let mut other_key_words = vec![
        // Control flow
        String::from("for"),
    ];

    other_key_words.extend(command_names);
    other_key_words


}

/// Static list of seperator keywords
pub fn seperators() -> Vec<&'static str> {
    vec![
        // Compound commands
        "&&", "||", ";", "\r\n", "\n",
        // Control flow
        "}",
    ]
}