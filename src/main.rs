use std::io::{self, Write};

fn main() {
    loop {
        print_prompt();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Cannot read line"); 
   
        match line.trim() {
            ".exit" => return,
            _ => println!("Unrecognized command '{}'", line.trim()),
        }
    }
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap()
}
