pub enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

pub enum MetaCommand {
    Exit,
}

impl MetaCommand {
    pub fn match_cmd_enum(s: String) -> Option<MetaCommand> {
        match s.as_str() {
            ".exit" => Some(MetaCommand::Exit),
            _ => None,
        }
    }

    pub fn do_cmd(mc: MetaCommand) -> MetaCommandResult {
        match mc {
            MetaCommand::Exit => std::process::exit(0),
            _ => MetaCommandResult::UnrecognizedCommand,
        }
    }
}
