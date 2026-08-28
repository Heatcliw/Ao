pub enum Command {
    Exit,
    Push,
    Send,
    File(String),
    Rule(String),
    Identity(String),
    Personality(String),
    Style(String),
    Unknown,
}

pub fn check_command(input: &str) -> Command {
    
    if let Some(value) = input.strip_prefix(":iden ") {
        return Command::Identity(value.to_string());
    }

    if let Some(value) = input.strip_prefix(":pers ") {
        return Command::Personality(value.to_string());
    }

    if let Some(value) = input.strip_prefix(":rule ") {
        return Command::Rule(value.to_string());
    }

    if let Some(value) = input.strip_prefix(":style ") {
        return Command::Style(value.to_string());
    }

    if let Some(path) = input.strip_prefix(":file ") {
        return Command::File(path.to_string());
    }
    
    match input.trim() {
        ":exit" => Command::Exit,
        ":push" => Command::Push,
        ":send" => Command::Send,
        _ => Command::Unknown,
    }
}
