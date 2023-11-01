mod error;
mod parser_core;

use parser_core::scanner::Scanner;

use std::env;
use std::fs;
use std::io;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        return Err(());
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }

    Ok(())
}

fn run_file(path: &String) {
    match fs::read_to_string(path) {
        Ok(source) => run(&source),
        _ => println!("Error opening file"),
    };
}

fn run_prompt() {
    loop {
        print!("> ");

        io::Write::flush(&mut io::stdout()).expect("run_prompt_flush_error");
        let mut line = String::new();

        // read_line returns Result<String> no Err cannot use '?'
        if let Ok(_) = io::stdin().read_line(&mut line) {
            if line.trim() == "exit" {
                break;
            }
            run(line.trim());
        }
    }
}

fn run(s: &str) {
    let mut scanner = Scanner::new(s);
    if let Ok(tokens) = scanner.scan_tokens() {
        for token in tokens {
            println!("{token}");
        }
    }
}
