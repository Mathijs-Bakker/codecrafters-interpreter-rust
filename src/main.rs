use crate::lexer::analyzer::Scanner;
use std::env;
use std::fs;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let mut scanner_error = false;

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let tokens = Scanner::new(&file_contents);

            for token in tokens {
                let token = match token {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("{e}");
                        scanner_error = true;
                        continue;
                    }
                };
                println!("{token}");
            }
            println!("EOF  null");

            if scanner_error {
                std::process::exit(65);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
