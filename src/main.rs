use std::process::exit;

use util::{print_prompt, InputBuffer};

mod util;

fn main() {
    let mut ib = InputBuffer::new();
    loop {
        print_prompt();
        ib.read_line();

        if ib.buffer_to_string() == ".exit".to_string() {
            ib.free_input_buffer();
            break;
        } else {
            println!("ERROR: Unreocgnized command.");
        }
    }
}
