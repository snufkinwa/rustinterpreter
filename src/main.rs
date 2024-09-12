use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;
use bytes::Bytes;  

use codecraftersinterpreter::token::tokenizer::tokenize;
use codecraftersinterpreter::parser::parser::Parser;
use codecraftersinterpreter::parser::ast_printer::AstPrinter; 
use codecraftersinterpreter::interpreter::interpreter::Interpreter;
use codecraftersinterpreter::token::output::print_tokens_and_errors;

fn main() {
    writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} <command> <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    // Read the file contents as bytes
    let file_bytes = fs::read(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        Vec::new()
    });

    // Convert the Vec<u8> to Bytes
    let file_bytes = Bytes::from(file_bytes);

    match command.as_str() {
        "tokenize" => {
            // Call the tokenizer and handle the Result
            match tokenize(file_bytes) {
                Ok((tokens, errors)) => {
                    // Pass references to the tokens and errors vectors
                    print_tokens_and_errors(&tokens, &errors);
        
                    if !errors.is_empty() {
                        exit(65); // Exit with code 65 if errors exist
                    }
                }
                Err(e) => {
                    eprintln!("Failed to tokenize input: {}", e);
                    exit(65);
                }
            }
        },

        "parse" => {
            // Tokenize the input
            match tokenize(file_bytes) {
                Ok((tokens, errors)) => {
                    if !errors.is_empty() {
                        for error in &errors {
                            eprintln!("{}", error);
                        }
                        exit(65); // Exit if errors were encountered during tokenization
                    }

                    // Continue to parsing if no errors
                    let mut parser = Parser::new(tokens);
                    match parser.parse() {
                        Ok(statements) => {
                            let printer = AstPrinter;

                            for statement in statements.iter() {
                                let output = printer.print_stmt(statement);
                                println!("{}", output);
                            }
                        }
                        Err(e) => {
                            writeln!(io::stderr(), "Parse error: {}", e).unwrap();
                            exit(65);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to tokenize input: {}", e);
                    exit(65);
                }
            }
        },

        "evaluate" => {
            // Tokenize the input
            match tokenize(file_bytes) {
                Ok((tokens, errors)) => {
                    if !errors.is_empty() {
                        for error in &errors {
                            eprintln!("{}", error);
                        }
                        exit(65); // Exit if errors were encountered during tokenization
                    }

                    // Continue to parsing and interpreting if no errors
                    let mut parser = Parser::new(tokens);
                    match parser.parse() {
                        Ok(statements) => {
                            // Create an interpreter instance and interpret the parsed statements
                            let mut interpreter = Interpreter::new();
                            interpreter.interpret(statements);
                        }
                        Err(e) => {
                            writeln!(io::stderr(), "Parse error: {}", e).unwrap();
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

        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
