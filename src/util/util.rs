use std::io::{self, Write};

/// Prints `>` prompt
pub fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}
