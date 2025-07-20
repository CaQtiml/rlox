mod token;
mod scanner;
mod error;

use scanner::Scanner;
use error::ErrorReporter;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut error_reporter = ErrorReporter::new();

    match args.len() {
        1 => run_prompt(&mut error_reporter),
        2 => run_file(&args[1], &mut error_reporter),
        _ => {
            println!("Usage: lox [script]");
            process::exit(64);
        }
    }
}

fn run_file(path: &str, error_reporter: &mut ErrorReporter) {
    match fs::read_to_string(path) {
        Ok(source) => {
            run(source, error_reporter);
            if error_reporter.had_error() {
                process::exit(65);
            }
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            process::exit(66);
        }
    }
}

fn run_prompt(error_reporter: &mut ErrorReporter) {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                run(input, error_reporter);
                error_reporter.reset();
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
    }
}

fn run(source: String, error_reporter: &mut ErrorReporter) {
    let mut scanner = Scanner::new(source);
    
    match scanner.scan_tokens() {
        Ok(tokens) => {
            // For now, just print the tokens
            for token in tokens {
                println!("{}", token);
            }
        }
        Err(errors) => {
            eprintln!("{}", errors);
            error_reporter.report(0, "", "Scanning failed");
        }
    }
}