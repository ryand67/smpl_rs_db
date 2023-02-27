use std::num::ParseIntError;

pub const TABLE_MAX_ROWS: u32 = 100 * 4096;

pub enum TableErrors {
    TableFull,
}

#[derive(Default, Debug)]
pub struct Table {
    pub num_rows: usize,
    pub pages: Vec<Page>,
}

#[derive(Default, Debug)]
pub struct Page {
    pub rows: Vec<Row>,
}

#[derive(Default, Debug)]
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
}
