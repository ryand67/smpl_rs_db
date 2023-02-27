use super::{CommandError, Row, Table, TABLE_MAX_ROWS};

pub enum StatementType {
    Insert,
    Select,
}

pub struct Statement<'a> {
    pub content: String,
    pub s_type: Option<StatementType>,
    pub args: Vec<String>,
    pub row_to_insert: Row,
    pub table: &'a Table,
}

impl<'a> Statement<'a> {
    pub fn new(content: String, table: &'a Table) -> Self {
        Self {
            s_type: None,
            content,
            args: Vec::new(),
            row_to_insert: Row::default(),
            table,
        }
    }

    pub fn prepare_statement(&mut self) -> Result<(), String> {
        if self.content.starts_with("insert") {
            self.s_type = Some(StatementType::Insert);

            let arg_buf: Vec<String> = self
                .content
                .split_whitespace()
                .map(|arg| arg.to_string())
                .collect();

            if arg_buf.len() > 3 {
                return Err("Too many args".to_string());
            }

            self.args = arg_buf;

            match Row::new_from_args(&self.args) {
                Ok(row) => self.row_to_insert = row,
                Err(e) => println!("{e}"),
            }

            return Ok(());
        }

        if self.content.starts_with("select") {
            self.s_type = Some(StatementType::Select);
            return Ok(());
        }

        return Err("Unrecognized Statement".to_string());
    }

    pub fn execute_statement(&mut self) {
        match &self.s_type {
            Some(t) => match t {
                StatementType::Insert => println!("Insert here"),
                StatementType::Select => println!("Select here"),
            },
            None => todo!(),
        }
    }

    fn execute_insert(&mut self) -> Result<(), CommandError> {
        if self.table.num_rows > TABLE_MAX_ROWS as usize {
            return Err(CommandError::TableFull);
        }

        return Ok(());
    }
}
