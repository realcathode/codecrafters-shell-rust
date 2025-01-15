#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();

    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
    
        stdin.read_line(&mut input).unwrap();
    
        match input.trim() {
            "exit 0" => break,
            _ => {
                println!("{}: command not found", input.trim());    
            }
        }
        io::stdout().flush().unwrap();
    }
}
