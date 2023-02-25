use std::io::{self, Write};

pub fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}
