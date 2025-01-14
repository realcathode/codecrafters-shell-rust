#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    let stdin = io::stdin();

    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
    
        stdin.read_line(&mut input).unwrap();
        let args: Vec<&str> = input.trim().split_ascii_whitespace().collect();
        match args[0].trim() {
            "exit" => {
                match args.get(1) {
                    Some(&v) => match v.parse::<i32>() {
                        Ok(code) => {
                            // println!("Exiting with code {}", code);
                            process::exit(code);
                        },
                        Err(_) => {
                            // println!("Invalid exit code");
                            continue;
                        }
                    },
                    None => process::exit(0),
                }
            },
            "echo" => {
                for i in 1..args.len() {
                    match args.get(i) {
                        Some(&s) => {
                            print!("{}", s);
                        },
                        None => {
                            println!("args[{}] out of bounds", i);
                        }
                    }
                    print!(" ");
                }
                println!("");
            },
            _ => {
                println!("{}: command not found", input.trim());    
            }
        }   
        io::stdout().flush().unwrap();
    }
}
