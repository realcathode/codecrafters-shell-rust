#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;
use std::env;
use std::path::Path;
use std::process::Command;

const BUILTIN: [&str; 3] = ["echo", "exit", "type"];

fn find_executable(s: &str) -> Option<PathBuf> {
    if let Ok(path_env) = env::var("PATH") {
        let paths: Vec<&str> = path_env.trim().split(":").collect();
        for path in paths {
            let full_path = Path::new(path).join(s);
            if full_path.exists() && full_path.is_file() {
                return Some(full_path);
            }
        }
    }
    None
}


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
                    println!("{} is a shell builtin", s);
                } else if let Ok(path_env) = env::var("PATH") {
                    let paths: Vec<&str> = path_env.trim().split(':').collect();
                    let mut found = false;

                    for path in paths {
                        let full_path = format!("{}/{}", path, s);
                        if Path::new(&full_path).exists() {
                            println!("{} is {}", s, full_path);
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        println!("{}: not found", s);
                    }
                } else {
                    println!("PATH environment variable is not set.");
                }
            },
            None => {}
        }

        if i != args.len() - 1 {
            print!(" ");
        }
    }
    // println!("");
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
                if let Some(path) = find_executable(args[0]) {
                    if let Some(executable_name) = Path::new(&path)
                        .file_name()
                        .and_then(|os_str| os_str.to_str()) {
                        Command::new(executable_name)
                            .args(&args[1..])
                            .status()
                            .expect("failed to execute process");
                    } else {
                        println!("could not extract the executable name.");
                    }
                } else {
                    println!("{}: command not found", args[0]);
                }
            }
        }   
        io::stdout().flush().unwrap();
    }
}
