use std::io::{self, Write};

#[macro_use]
macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

const PAGE_SIZE : u32 = 4096;
const ROW_SIZE : u32 = std::mem::size_of::<Row>() as u32;
const TABLE_MAX_PAGES : u32 = 100;
const ROWS_PER_PAGE : u32 = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS : u32 = ROWS_PER_PAGE * TABLE_MAX_PAGES ;

enum StatementType {
    Insert,
    Select,
}

struct Statement {
    statement_type: StatementType,
    row_to_insert: Option<Row>,
}

struct Row {
    id: u32,
    username: String,
    email: String,
}

struct Table {
    pages: Vec<Page>,
    num_rows: u32,
}

type Page = Vec<Row>;

fn main() {
    let mut table : Table = Table{
        pages: Vec::with_capacity(TABLE_MAX_PAGES as usize),
        num_rows: 0,
    };
    table.pages.insert(0, Page::with_capacity(1));

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
                execute_statement(stmt, &mut table);
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
                statement_type: StatementType::Select,
                row_to_insert: None,
            })
        },
        "insert" => {
            let output = scan!(line, char::is_whitespace, String, u32, String, String);

            println!("{:?}", output);
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

fn execute_statement(statement : Statement, table : &mut Table) -> Result<(), String>{
    match statement.statement_type {
        StatementType::Insert => {
            return  execute_insert(statement, table);
        },
        StatementType::Select => {
            return execute_select(statement, table);
        },
    }
}

struct Slot {
    page: usize,
    page_index: usize,
}

fn row_slot(table: &mut Table, row_num: u32) -> Slot {
    let page_num = row_num / ROWS_PER_PAGE;

    let page = match table.pages.get(page_num as usize) {
        Some(page) => &page,
        None => {
            table.pages.insert(page_num as usize, Page::with_capacity(ROWS_PER_PAGE as usize));
            &table.pages[page_num as usize]
        },
    };

    let page_index = row_num % ROWS_PER_PAGE;
    return Slot{
        page_index: page_index as usize,
        page: page_num as usize,
    }
}

fn execute_insert(statement: Statement, table: &mut Table) -> Result<(), String> {
    if table.num_rows >= TABLE_MAX_ROWS {
        return Err(String::from("table full"));
    }

    if let Some(row_to_insert) = statement.row_to_insert {
        let slot = row_slot(table, table.num_rows);
        table.pages[slot.page].insert(slot.page_index, row_to_insert);
    } else {
        panic!("execute_insert without row to insert")
    }

    return Ok(())
}

fn execute_select(statement: Statement, table: &mut Table) -> Result<(), String> {
    for page in &table.pages {
        for row in page {
            println!("({}, {}, {})", row.id, row.username, row.email)
        }
    }

    return Ok(())
}