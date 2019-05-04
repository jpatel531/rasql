use super::row::*;
use super::constants::*;
use super::table::Table;

#[macro_use]
macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

enum StatementType {
    Insert,
    Select,
}

pub struct Statement {
    statement_type: StatementType,
    row_to_insert: Option<Row>,
}

impl Statement {
    pub fn prepare(line: &str) -> Result<Statement, String> {
        if line.chars().count() <= 6 {
            return Err(String::from("length of line is <= 6"));
        }

        match &line.to_lowercase()[..6] {
            "select" => {
                return Ok(Statement{
                    statement_type: StatementType::Select,
                    row_to_insert: None,
                })
            },
            "insert" => {
                let output = scan!(line, char::is_whitespace, String, u32, String, String);

                let (_, id, username, email) = output;
                if id.is_none() || username.is_none() || email.is_none() {
                    return Err(String::from("syntax error"));
                }

                return Ok(Statement{
                    statement_type: StatementType::Insert,
                    row_to_insert: Some(Row{
                        id: id.unwrap(),
                        username: username.unwrap(),
                        email: email.unwrap(),
                    })
                })
            },
            _ => {
                return Err(String::from("Unrecognized statement"));
            }
        }
    }

    pub fn execute(self, table : &mut Table) -> Result<(), String>{
        match self.statement_type {
            StatementType::Insert => {
                return self.execute_insert(table);
            },
            StatementType::Select => {
                return self.execute_select(table);
            },
        }
    }

    fn execute_insert(self, table: &mut Table) -> Result<(), String> {
        if let Some(row_to_insert) = self.row_to_insert {
            // TODO implement max cells

            let mut cursor = table.end();
            cursor.insert_leaf_node(row_to_insert.id, row_to_insert);
        } else {
            panic!("execute_insert without row to insert")
        }

        return Ok(())
    }

    fn execute_select(self, table: &mut Table) -> Result<(), String> {
        let mut cursor = table.start();
        cursor.iterate(|row: &Row| {
            println!("({}, {}, {})", row.id, row.username, row.email);
        });
        return Ok(())
    }

}

