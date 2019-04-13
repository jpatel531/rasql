use std::io::{self, Write};

enum StatementType {
    Insert,
    Select,
}

struct Statement {
    statement_type: StatementType,
}

fn main() {
    loop {
        print_prompt();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Cannot read line"); 
        
        let trimmed_lime : &str = line.trim();
        if trimmed_lime.chars().next().unwrap() == '.' {
            match do_meta_command(trimmed_lime) {
                Ok(_) => continue,
                Err(msg) => { 
                    println!("{}", msg);
                    continue;
                }
            }
        }

        match prepare_statement(trimmed_lime) {
            Err(msg) => {
                println!("{}", msg)
            },
            Ok(stmt) => {
                execute_statement(stmt);
            },
        }
    }
}

fn do_meta_command(line : &str) -> Result<(), String> {
    match line {
        ".exit" => std::process::exit(0),
        _ =>  {
            return Err(format!("Unrecognized command '{}'", line));
        },
    }  
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap()
}

fn prepare_statement(line: &str) -> Result<Statement, String> {
    if line.chars().count() <= 6 {
        return Err(String::from("length of line is <= 6"));
    }

    match &line.to_lowercase()[..6] {
        "select" => {
            return Ok(Statement{
                statement_type: StatementType::Select
            })
        },
        "insert" => {
            return Ok(Statement{
                statement_type: StatementType::Insert
            })
        },
        _ => {
            return Err(String::from("Unrecognized statement"));
        }
    }
}

fn execute_statement(statement : Statement) {
    match statement.statement_type {
        StatementType::Insert => {
            println!("This is where we would do an insert")
        },
        StatementType::Select => {
            println!("This is where we would do a select")
        },
    }
}