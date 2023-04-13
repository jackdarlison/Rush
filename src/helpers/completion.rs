use crate::helpers::commands::commands;


pub fn complete_command(command: String) -> Vec<String> {
    keywords().into_iter().filter(|c| c.starts_with(command.as_str())).collect()
}

pub fn keywords() -> Vec<String> {
    let command_names = commands().into_iter().map(|c| c.name().to_owned());
    
    let mut other_key_words = vec![
        // Control flow
        String::from("for"),
    ];

    other_key_words.extend(command_names);
    other_key_words


}

pub fn seperators() -> Vec<&'static str> {
    vec![
        // Compound commands
        "&&", "||", ";", "\r\n", "\n",
        // Control flow
        "}",
    ]
}