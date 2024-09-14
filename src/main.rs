use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;
use bytes::Bytes;

use codecraftersinterpreter::token::tokenizer::tokenize;
use codecraftersinterpreter::parser::parser::Parser;
use codecraftersinterpreter::parser::ast_printer::AstPrinter;
use codecraftersinterpreter::interpreter::interpreter::Interpreter;
use codecraftersinterpreter::interpreter::runtime_error::InterpreterError;
use codecraftersinterpreter::token::output::print_tokens_and_errors;


fn run(source: Bytes, require_semicolon: bool, evaluate_mode: bool) -> Result<(), InterpreterError> {
    // Tokenize the source
    let (tokens, errors) = tokenize(source).unwrap();

    // Log tokenizer errors but do not exit immediately
    if !errors.is_empty() {
        for error in &errors {
            eprintln!("Tokenizer error: {}", error);
        }
    }

    // Pass the tokens to the parser, continue even if there were tokenizer errors
    let mut parser = Parser::new(tokens, require_semicolon);
    let statements = match parser.parse() {
        Ok(statements) => statements,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            exit(65); // Exit with 65 for parse errors
        }
    };

    // Interpret the parsed statements
    let mut interpreter = Interpreter::new(evaluate_mode);
    if let Err(e) = interpreter.interpret(statements) {
        eprintln!("Runtime error: {}", e);
        exit(70); // Exit with 70 for runtime errors
    }

    Ok(())
}

fn main() -> Result<(), InterpreterError> {
    // Log message at the start
    writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} <command> <filename>", args[0]).unwrap();
        return Ok(());
    }

    // Extract command and filename
    let command = &args[1];
    let filename = &args[2];

    // Read the file contents as bytes
    let file_bytes = fs::read(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        Vec::new()
    });
    let file_bytes = Bytes::from(file_bytes);  

    // Match the command and handle each case
    match command.as_str() {
        "tokenize" => {
            // Tokenize the input and print tokens and errors
            match tokenize(file_bytes) {
                Ok((tokens, errors)) => {
                    print_tokens_and_errors(&tokens, &errors);
                    if !errors.is_empty() {
                        exit(65);  
                    }
                }
                Err(e) => {
                    eprintln!("Failed to tokenize input: {}", e);
                    exit(65); 
                }
            }
        }

        "parse" => {
            // Tokenize the input and parse the tokens
            match tokenize(file_bytes) {
                Ok((tokens, errors)) => {
                    if !errors.is_empty() {
                        for error in &errors {
                            eprintln!("{}", error);
                        }
                        exit(65);  // Exit if tokenizer errors exist
                    }

                    // Parse the tokens and print the AST
                    let mut parser = Parser::new(tokens, false);  
                    match parser.parse() {
                        Ok(statements) => {
                            let printer = AstPrinter;
                            for statement in statements.iter() {
                                let output = printer.print_stmt(statement);
                                println!("{}", output);
                            }
                        }
                        Err(e) => {
                            eprintln!("Parse error: {}", e);
                            exit(65);  
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to tokenize input: {}", e);
                    exit(65); 
                }
            }
        }

        "evaluate" => {
            // Do not require semicolons in "evaluate" mode
            if let Err(e) = run(file_bytes, false, true) {
                eprintln!("{}", e);
            }
        }

        "run" => {
            // Require semicolons in "run" mode
            if let Err(e) = run(file_bytes, true, false) {
                eprintln!("{}", e);
            }
        }


        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
        }
    }

    Ok(())
}
