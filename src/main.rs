use util::{print_prompt, CommandError, InputBuffer, MetaCommand, MetaCommandResult, Statement};

mod util;

fn main() {
    let mut ib = InputBuffer::new();
    loop {
        print_prompt();
        ib.read_line();

        if ib.is_meta_cmd() {
            if let Some(mc) = MetaCommand::match_cmd_enum(ib.buffer_to_string.to_string()) {
                match MetaCommand::do_cmd(mc) {
                    MetaCommandResult::Success => continue,
                    MetaCommandResult::UnrecognizedCommand => {
                        CommandError::UnrecognizedCommand(ib.buffer_to_string.clone()).log();
                        continue;
                    }
                }
            } else {
                println!("Unrecognized Meta Command");
                continue;
            }
        }

        let mut statement = Statement::new(ib.buffer_to_string.clone());

        match statement.prepare_statement() {
            Ok(_) => statement.execute_statement(),
            Err(e) => println!("{}", e),
        }
    }
}
