mod token;
mod scanner;
mod error;
mod expr;
mod ast_printer;
mod parser;
mod interpreter;
mod value;

use scanner::Scanner;
use error::ErrorReporter;
use expr::Expr;
use ast_printer::AstPrinter;
use token::{Token, TokenType, LiteralValue};
use parser::Parser;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;
use interpreter::Interpreter;

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
            if args[1] == "--test-parser" {
                test_parser();
                return;
            }
            if args[1] == "--test-interpreter" {
                test_interpreter();
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
            let mut parser = Parser::new(tokens.clone());
            if let Some(expr) = parser.parse(error_reporter) {
                // let mut printer = AstPrinter::new();
                // println!("{}", printer.print(&expr));
                let mut interpreter = Interpreter::new();
                match interpreter.interpret(&expr) {
                    Ok(value) => println!("{}", value),
                    Err(err) => {
                        if let Some(runtime_err) = err.downcast_ref::<interpreter::RuntimeError>() {
                            eprintln!("{}", runtime_err);
                        } else {
                            eprintln!("Runtime error: {}", err);
                        }
                    }
                }
            }
            // If parse returned None, errors were already reported
        }
        Err(errors) => {
            eprintln!("{}", errors);
            error_reporter.report(0, "", "Scanning failed");
        }
    }
}

// Add a test function
fn test_interpreter() {
    println!("Testing Interpreter...");
    
    let test_cases = vec![
        "1 + 2",
        "3 * 4 - 2", 
        "10 / 2",
        "(1 + 2) * 3",
        "\"hello\" + \" world\"",
        "\"num: \" + 42",
        "true == false",
        "!(5 > 3)",
        "3 >= 3",
        "nil == nil",
        // Error cases you can try:
        // "\"hello\" - \"world\"",  // Should give runtime error
        // "5 / 0",                 // Should give runtime error  
    ];
    
    for test_case in test_cases {
        println!("\n--- Evaluating: {} ---", test_case);
        let mut error_reporter = ErrorReporter::new();
        run(test_case.to_string(), &mut error_reporter);
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

fn test_parser() {
    println!("Testing Parser...");
    
    let test_cases = vec![
        "1 + 2 * 3",
        "(1 + 2) * 3", 
        "-123 * 45.67",
        "1 == 2 != 3",
        "!(1 < 2)",
        "\"hello\" + \"world\"",
        // Error cases
        "1 + + 2",
        "(1 + 2",
        "* 5",
    ];
    
    for test_case in test_cases {
        println!("\n--- Testing: {} ---", test_case);
        let mut error_reporter = ErrorReporter::new();
        let mut scanner = Scanner::new(test_case.to_string());
        
        match scanner.scan_tokens() {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens.clone());
                if let Some(expr) = parser.parse(&mut error_reporter) {
                    let mut printer = AstPrinter::new();
                    println!("Result: {}", printer.print(&expr));
                } else {
                    println!("Parse failed (errors reported above)");
                }
            }
            Err(err) => {
                println!("Scan error: {}", err);
            }
        }
    }
}