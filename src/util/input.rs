use std::process::exit;

pub struct InputBuffer {
    pub buffer: Vec<char>,
    pub buffer_length: usize,
    pub input_length: usize,
    pub buffer_to_string: String,
}

impl InputBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            buffer_length: 0,
            input_length: 0,
            buffer_to_string: String::new(),
        }
    }

    pub fn is_meta_cmd(&self) -> bool {
        if self.buffer_to_string.starts_with('.') {
            true
        } else {
            false
        }
    }

    pub fn read_line(&mut self) {
        let mut buf = String::new();
        match std::io::stdin().read_line(&mut buf) {
            Ok(n) => {
                if n <= 0 {
                    println!("Error reading input.");
                    exit(1);
                }

                self.input_length = n - 1;
                self.buffer = buf.chars().collect();

                let mut s: String = self.buffer.clone().into_iter().collect();
                if s.ends_with('\n') {
                    s.remove(s.len() - 1);
                }

                self.buffer_to_string = s;
            }
            Err(e) => {
                println!("{:?}", e);
                exit(1);
            }
        }
    }

    pub fn free_input_buffer(&mut self) {
        drop(self);
    }
}
