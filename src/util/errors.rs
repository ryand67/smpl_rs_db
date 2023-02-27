#[derive(Debug)]
pub enum CommandError {
    UnrecognizedCommand(String),
    TableFull,
}

impl CommandError {
    pub fn log(self) {
        match self {
            Self::UnrecognizedCommand(s) => {
                println!("{}", format!("Command {} not recognized.", s));
            }
            Self::TableFull => {
                println!("DB table at capacity.");
            }
            _ => {
                println!("Error")
            }
        }
    }
}
