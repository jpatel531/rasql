use std::env;
use std::io::{self, Write};
use std::str;

#[macro_use]
extern crate serde_derive;
extern crate bincode;

mod cursor;
mod btree;
mod row;
mod table;
mod statement;
mod pager;
mod constants;

use btree::*;
use cursor::*;
use row::*;
use table::*;
use statement::*;
use pager::*;
use constants::*;

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
            match do_meta_command(trimmed_lime, &mut table) {
                Ok(_) => continue,
                Err(msg) => {
                    println!("{}", msg);
                    continue;
                }
            }
        }

        match Statement::prepare(trimmed_lime) {
            Err(msg) => {
                println!("{}", msg)
            },
            Ok(stmt) => {
                stmt.execute(&mut table).expect("statement error");
            },
        }
    }
}

fn do_meta_command(line : &str, table: &mut Table) -> Result<(), String> {
    match line {
        ".exit" => {
            table.close();
            std::process::exit(0)
        },
        ".btree" => {
            let node = table.pager.get_page(0);
            println!("leaf (size {})", node.num_cells);
            for (i, key) in node.keys.iter().enumerate() {
                println!("  - {} : {}", i, key);
            }
            return Ok(())
        },
        _ =>  {
            return Err(format!("Unrecognized command '{}'", line));
        },
    }
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap()
}

fn db_open<'a>(filename: String) -> Result<Table, String> {
    match Pager::open(filename) {
        Ok(pager) => {
            return Ok(Table{
                pager: pager,
                root_page_num: 0,
            })
        }
        Err(err) => return Err(err),
    }
}
