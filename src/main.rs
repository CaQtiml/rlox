mod token;
mod scanner;
mod error;
mod expr;
mod ast_printer;

use scanner::Scanner;
use error::ErrorReporter;
use expr::Expr;
use ast_printer::AstPrinter;
use token::{Token, TokenType, LiteralValue};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut error_reporter = ErrorReporter::new();

    match args.len() {
        1 => run_prompt(&mut error_reporter),
        2 => {
            if args[1] == "--test-ast" {
                test_ast_printer();
                return;
            }
            run_file(&args[1], &mut error_reporter);
        }
        _ => {
            println!("Usage: lox [script] or lox --test-ast");
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

// Test function to demonstrate AST creation and printing
fn test_ast_printer() {
    println!("Testing AST Printer...");
    
    // Create expression: (* (- 123) (group 45.67))
    let expression = Expr::binary(
        Expr::unary(
            Token::new(TokenType::Minus, "-".to_string(), None, 1),
            Expr::literal(Some(LiteralValue::Number(123.0))),
        ),
        Token::new(TokenType::Star, "*".to_string(), None, 1),
        Expr::grouping(Expr::literal(Some(LiteralValue::Number(45.67)))),
    );

    let mut printer = AstPrinter::new();
    let result = printer.print(&expression);
    println!("AST: {}", result);
    
    // Test another expression: (== (+ 1 2) (- 4 3))
    let expression2 = Expr::binary(
        Expr::binary(
            Expr::literal(Some(LiteralValue::Number(1.0))),
            Token::new(TokenType::Plus, "+".to_string(), None, 1),
            Expr::literal(Some(LiteralValue::Number(2.0))),
        ),
        Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
        Expr::binary(
            Expr::literal(Some(LiteralValue::Number(4.0))),
            Token::new(TokenType::Minus, "-".to_string(), None, 1),
            Expr::literal(Some(LiteralValue::Number(3.0))),
        ),
    );
    
    let result2 = printer.print(&expression2);
    println!("AST: {}", result2);
}