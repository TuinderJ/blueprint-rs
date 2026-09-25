#[derive(Debug, PartialEq)]
pub enum Command {
    Help,
    Reset,
    Structs,
    Enums,
    Traits,
    Workflow,
}

pub fn parse_arguments() -> Vec<Command> {
    let args: Vec<String> = std::env::args().collect();

    args.iter()
        .filter_map(|arg| {
            if arg == "--help" || arg == "-h" {
                Some(Command::Help)
            } else if arg == "--reset" {
                Some(Command::Reset)
            } else if arg == "structs" {
                Some(Command::Structs)
            } else if arg == "enums" {
                Some(Command::Enums)
            } else if arg == "traits" {
                Some(Command::Traits)
            } else if arg == "check" {
                Some(Command::Workflow)
            } else {
                None
            }
        })
        .collect()
}
