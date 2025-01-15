use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{self, Command};

const BUILTIN: [&str; 4] = ["echo", "exit", "type", "pwd"];

fn find_executable(command: &str) -> Option<PathBuf> {
    env::var("PATH")
        .ok()?
        .split(':')
        .map(Path::new)
        .map(|path| path.join(command))
        .find(|path| path.exists() && path.is_file())
}

fn sh_exit(args: &[&str]) {
    let code = args.get(1).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
    process::exit(code);
}

fn sh_echo(args: &[&str]) {
    let output = args.iter().skip(1).cloned().collect::<Vec<_>>().join(" ");
    println!("{}", output);
}

fn sh_type(args: &[&str]) {
    for &arg in args.iter().skip(1) {
        if BUILTIN.contains(&arg) {
            println!("{} is a shell builtin", arg);
        } else if let Some(path) = find_executable(arg) {
            println!("{} is {}", arg, path.display());
        } else {
            println!("{}: not found", arg);
        }
    }
}

fn sh_pwd() {
    match env::current_dir() {
        Ok(v) => println!("{}", v.display()),
        Err(e) => println!("{}", e),
    }
}

fn execute_command(command: &str, args: &[&str]) {
    if let Some(path) = find_executable(command) {
        let binary_name = Path::new(&path).file_name().unwrap().to_str().unwrap();
        
        if let Err(e) = Command::new(binary_name).args(args).status() {
            eprintln!("Error executing {}: {}", command, e);
        }
    } else {
        println!("{}: command not found", command);
    }
}

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() || input.trim().is_empty() {
            continue;
        }

        let args: Vec<&str> = input.trim().split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        match args[0] {
            "exit" => sh_exit(&args),
            "echo" => sh_echo(&args),
            "type" => sh_type(&args),
            "pwd" => sh_pwd(),
            command => execute_command(command, &args[1..]),
        }
    }
}
