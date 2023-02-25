pub enum CommandError {
    UnrecognizedCommand(String),
}

impl CommandError {
    pub fn log(self) {
        match self {
            CommandError::UnrecognizedCommand(s) => {
                println!("{}", format!("Command {} not recognized.", s));
            }
        }
    }
}
