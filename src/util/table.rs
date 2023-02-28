use std::{fs::File, num::ParseIntError, path::PathBuf};

pub const TABLE_MAX_ROWS: u32 = 100 * 4096;

pub enum TableErrors {
    TableFull,
}

#[derive(Debug)]
pub struct Table {
    // TODO: Make this an array probably
    pub rows: Vec<Row>,
    pub pager: File,
}

impl Table {
    pub fn new(path: PathBuf) -> Self {
        let pager = File::open(path).expect("Error opening file");
        Self {
            rows: Vec::new(),
            pager,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Row {
    id: usize,
    username: String,
    email: String,
}

impl Row {
    pub fn new(id: usize, username: String, email: String) -> Self {
        Self {
            id,
            username,
            email,
        }
    }

    pub fn new_from_args(args: &Vec<String>) -> Result<Self, ParseIntError> {
        match args[0].parse::<usize>() {
            Ok(id) => Ok(Self {
                id,
                username: args[1].clone(),
                email: args[2].clone(),
            }),
            Err(e) => Err(e),
        }
    }

    pub fn print_row(&self) {
        println!(
            "id: {} - username: {} - email: {}",
            self.id, self.username, self.email
        );
    }
}
