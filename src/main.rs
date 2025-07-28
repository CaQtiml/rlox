mod token;
mod scanner;
mod error;
mod expr;
mod ast_printer;
mod parser;
mod interpreter;
mod value;
mod stmt;
mod environment;
mod function;

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
            if args[1] == "--test-control-flow" {
                test_control_flow();
                return;
            }
            run_file(&args[1], &mut error_reporter);
        }
        _ => {
            println!("Usage: lox [script] or lox --test-ast or lox --test-control-flow");
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

fn run(source: String, error_reporter: &mut ErrorReporter) {
    let mut scanner = Scanner::new(source);
    
    match scanner.scan_tokens() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens.clone());
            if let Some(statements) = parser.parse(error_reporter) {
                let mut interpreter = Interpreter::new();
                if let Err(err) = interpreter.interpret(&statements) {
                    if let Some(runtime_err) = err.downcast_ref::<interpreter::RuntimeError>() {
                        eprintln!("{}", runtime_err);
                    } else {
                        eprintln!("Runtime error: {}", err);
                    }
                }
            }
        }
        Err(errors) => {
            eprintln!("{}", errors);
            error_reporter.report(0, "", "Scanning failed");
        }
    }
}

fn run_prompt(error_reporter: &mut ErrorReporter) {
    let mut interpreter = Interpreter::new(); // Create interpreter once
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                run_repl(input, error_reporter, &mut interpreter); // Pass interpreter
                error_reporter.reset();
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
    }
}

fn run_repl(source: String, error_reporter: &mut ErrorReporter, interpreter: &mut Interpreter) {
    let mut scanner = Scanner::new(source);
    
    match scanner.scan_tokens() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens.clone());
            if let Some(statements) = parser.parse(error_reporter) {
                if let Err(err) = interpreter.interpret(&statements) {
                    if let Some(runtime_err) = err.downcast_ref::<interpreter::RuntimeError>() {
                        eprintln!("{}", runtime_err);
                    } else {
                        eprintln!("Runtime error: {}", err);
                    }
                }
            }
        }
        Err(errors) => {
            eprintln!("{}", errors);
            error_reporter.report(0, "", "Scanning failed");
        }
    }
}

fn test_control_flow() {
    println!("Testing Control Flow...");
    
    let test_cases = vec![
        // If statements
        "if (true) print \"hello\";",
        "if (false) print \"not printed\"; else print \"else executed\";",
        "if (1 > 2) print \"impossible\"; else print \"math works\";",
        
        // While loops
        "var i = 0; while (i < 3) { print i; i = i + 1; }",

        // For loops
        "for (var i = 0; i < 3; i = i + 1) print i;",
        "for (var i = 1; i < 10; i = i *2) print i;",
        "for (var i = 0; i < 3; i = i + 1) { print \"Count: \"; print i; }",

        // For loop with just condition
        "var j = 0; for (; j < 2; j = j + 1) print j;",
        
        // Logical operators (simple cases)
        "print true and false;",
        "print true or false;",
        "var a = true; var b = false; print a and b;",
        "var c = false; var d = true; print c or d;",
        
        // Nested control flow
        "var x = 5; if (x > 3) { var y = x * 2; while (y > 0) { print y; y = y - 1; } }",

        // Nested for loops
        "for (var i = 1; i <= 2; i = i + 1) { for (var j = 1; j <= 2; j = j + 1) { print i * j; } }",
        
        // Complex logical expressions
        "var a = true; var b = false; if (a and !b) print \"logic works\";",
    ];
    
    for test_case in test_cases {
        println!("\n--- Testing: {} ---", test_case);
        let mut error_reporter = ErrorReporter::new();
        run(test_case.to_string(), &mut error_reporter);
    }
}

fn test_interpreter() {
    println!("Testing Interpreter with statements...");
    
    let test_cases = vec![
        "print \"Hello, world!\";",
        "var a = 10;",
        "var b = 20; print a + b;",
        "var name = \"Alice\"; print \"Hello, \" + name + \"!\";",
        "var x; print x;", // Should print nil
        "1 + 2;", // Expression statement
        "var result = 3 * 4; print result;",
    ];
    
    for test_case in test_cases {
        println!("\n--- Executing: {} ---", test_case);
        let mut error_reporter = ErrorReporter::new();
        run(test_case.to_string(), &mut error_reporter);
    }
}

fn test_ast_printer() {
    println!("Testing AST Printer...");
    
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
        "print 1 + 2;",
        "var a = 5;",
        "print a;",
        "1 + 2;",
        "1 + 2;2+3;",
        "if (true) print \"hello\";",
        "while (false) print \"never\";",
        // Error cases
        "var;",
        "print",
    ];
    
    for test_case in test_cases {
        println!("\n--- Testing: {} ---", test_case);
        let mut error_reporter = ErrorReporter::new();
        let mut scanner = Scanner::new(test_case.to_string());
        
        match scanner.scan_tokens() {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens.clone());
                if let Some(statements) = parser.parse(&mut error_reporter) {
                    println!("Parsed {} statements successfully", statements.len());
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