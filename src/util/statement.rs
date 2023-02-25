pub enum StatementType {
    Insert,
    Select,
}

pub struct Statement {
    pub content: String,
    pub s_type: Option<StatementType>,
}

impl Statement {
    pub fn new(content: String) -> Self {
        Self {
            s_type: None,
            content,
        }
    }

    pub fn prepare_statement(&mut self) -> Result<(), String> {
        if self.content.starts_with("insert") {
            self.s_type = Some(StatementType::Insert);
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
}
