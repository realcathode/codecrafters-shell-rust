#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

const BUILTIN: [&str; 3] = ["echo", "exit", "type"];

fn sh_exit(args: Vec<&str>) {
    match args.get(1) {
        Some(&v) => match v.parse::<i32>() {
            Ok(code) => {
                process::exit(code);
            },
            Err(_) => {
                process::exit(1);
            }
        },
        None => process::exit(0),
    }
}

fn sh_echo(args: Vec<&str>) {
    for i in 1..args.len() {
        match args.get(i) {
            Some(&s) => {
                print!("{}", s);
            },
            None => {
                println!("args[{}] out of bounds", i);
            }
        }
        if i != args.len() - 1 {
            print!(" ");
        }
    }
    println!("");
}

fn sh_type(args: Vec<&str>) {
    for i in 1..args.len() {
        match args.get(i) {
            Some(&s) => {
                if BUILTIN.contains(&s) {
                    print!("{} is a shell builtin", s);
                }else {
                    print!("{}: not found", s);
                }
            },
            None => {
            },
        }
        if i != args.len() - 1 {
            print!(" ");
        }
    }
    println!("");
}

fn main() {
    let stdin = io::stdin();

    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
    
        stdin.read_line(&mut input).unwrap();
        if input.trim().is_empty() {
            continue;
        }
        
        let args: Vec<&str> = input.trim().split_ascii_whitespace().collect();

        
        match args[0].trim() {
            "exit" => {
                sh_exit(args);
            },
            "echo" => {
                sh_echo(args);
            },
            "type" => {
                sh_type(args);
            }
            _ => {
                println!("{}: command not found", input.trim());    
            }
        }   
        io::stdout().flush().unwrap();
    }
}
