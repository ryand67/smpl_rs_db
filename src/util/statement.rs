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
    pub table: &'a mut Table,
}

impl<'a> Statement<'a> {
    pub fn new(content: String, table: &'a mut Table) -> Self {
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
                .skip(1)
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
                // TODO: Handle Result
                StatementType::Insert => self.execute_insert().expect("Insert failed"),
                StatementType::Select => self.execute_select(),
            },
            None => todo!(),
        }
    }

    fn execute_insert(&mut self) -> Result<(), CommandError> {
        if self.table.rows.len() > TABLE_MAX_ROWS as usize {
            return Err(CommandError::TableFull);
        }

        self.table.rows.push(self.row_to_insert.to_owned());

        return Ok(());
    }

    fn execute_select(&mut self) {
        for row in &self.table.rows {
            row.print_row();
        }
    }
}
