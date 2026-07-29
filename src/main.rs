use std::io::{self, Write};
use std::process;

mod builtins;
mod parser;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // EOF (Ctrl+D)
                println!();
                break;
            }
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() {
                    continue;
                }

                let tokens = parser::parse(input);
                if tokens.is_empty() {
                    continue;
                }

                let command = &tokens[0];
                let args = &tokens[1..];

                match command.as_str() {
                    "echo" => builtins::echo(args),
                    "cd" => builtins::cd(args),
                    "pwd" => builtins::pwd(),
                    "exit" => process::exit(0),
                    _ => println!("Command '{}' not found", command),
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}
