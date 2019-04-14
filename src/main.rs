use std::env;
use std::io::{self, Write};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
#[macro_use]
extern crate serde_derive;
extern crate bincode;

use bincode::{serialize, deserialize};


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

#[derive(Serialize, Deserialize, Debug)]
struct Row {
    id: u32,
    username: String,
    email: String,
}

struct Table {
    pager: Pager,
    num_rows: u32,
}

type Page = Vec<Row>;

struct Slot {
    page: usize,
    page_index: usize,
}

struct Pager {
    file: File,
    pages: Vec<Page>
}

struct Cursor<'a> {
    table: &'a mut Table,
    row_num: u32,
    end_of_table: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Must supply a filename as argument");
        std::process::exit(1);
    }

    let filename = &args[1];

    let mut table : Table = db_open(filename.to_string()).unwrap();

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

fn cursor_value(cursor: &mut Cursor) -> Slot {
    let row_num = cursor.row_num;
    let page_num = row_num / ROWS_PER_PAGE;
    ensure_page(&mut cursor.table.pager, page_num);
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
        let mut cursor = table_end(table);
        let slot = cursor_value(&mut cursor);
        table.pager.pages[slot.page].insert(slot.page_index, row_to_insert);
        table.num_rows += 1;
    } else {
        panic!("execute_insert without row to insert")
    }

    return Ok(())
}

fn execute_select(statement: Statement, table: &mut Table) -> Result<(), String> {
    let mut cursor = table_start(table);
    while !cursor.end_of_table {
        let slot = cursor_value(&mut cursor);
        let row = &cursor.table.pager.pages[slot.page][slot.page_index];
        println!("({}, {}, {})", row.id, row.username, row.email);
        cursor_advance(&mut cursor)
    }

    return Ok(())
}

fn db_open(filename: String) -> Result<Table, String> {
    match pager_open(filename) {
        Ok(pager) => {
            let num_rows = pager.file.metadata().unwrap().len() as u32 / ROW_SIZE;
            return Ok(Table{
                pager: pager,
                num_rows: num_rows,
            })
        }
        Err(err) => return Err(err),
    }
}

fn pager_open(filename: String) -> Result<Pager, String> {
    match File::open(filename) {
        Ok(file) => {
            return Ok(Pager{
                file: file,
                pages: Vec::with_capacity(TABLE_MAX_PAGES as usize),
            })
        },
        Err(err) => {
            return Err(format!("{}", err))
        }
    }
}

fn ensure_page(pager: &mut Pager, page_num: u32) {
    if page_num > TABLE_MAX_PAGES {
        println!("Tried to fetch page number out of bounds. {} > {}", page_num, TABLE_MAX_PAGES);
        std::process::exit(1);
    }

    if pager.pages.get(page_num as usize).is_none() {
        // Cache miss, load from file
        let file = BufReader::new(&pager.file);

        let mut rows = Page::with_capacity(ROWS_PER_PAGE as usize);
        for (num, line) in file.lines().enumerate() {
            let l = line.unwrap();
            let chars : String = l.chars().collect();
            let buf = chars.as_bytes();
            let row : Row = deserialize(&buf).unwrap();
            rows.insert(num as usize, row);
        };
        pager.pages.insert(page_num as usize, rows);
    }
}

fn table_start(table: &mut Table) -> Cursor {
    let end_of_table = table.num_rows == 0;
    return Cursor{
        table: table,
        row_num: 0,
        end_of_table: end_of_table,
    }
}

fn table_end(table: &mut Table) -> Cursor {
    let row_num = table.num_rows;
    return Cursor{
        table: table,
        row_num: row_num,
        end_of_table: true,
    }
}

fn cursor_advance(cursor: &mut Cursor) {
    cursor.row_num += 1;
    if cursor.row_num >= cursor.table.num_rows {
        cursor.end_of_table = true
    }
}