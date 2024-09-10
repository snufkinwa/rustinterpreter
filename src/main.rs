use std::env;
use std::fs;
use std::io::{self, Write};

use codecraftersinterpreter::token::tokenizer::tokenize;
use codecraftersinterpreter::scanner::scanner::Scanner;
use codecraftersinterpreter::parser::parser::Parser;
use codecraftersinterpreter::parser::ast_printer::AstPrinter; 

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} <command> <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            //call function tokenize
            tokenize(&file_contents);
        },

        "scanner" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let mut scanner = Scanner::new(file_contents);
            let tokens = scanner.scan_tokens();

            // Print each token for debugging purposes
            for token in tokens {
                println!("{:?}", token);
            }
        },

        "parse" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let mut scanner = Scanner::new(file_contents);
            let tokens = scanner.scan_tokens();

            let mut parser = Parser::new(tokens);
            let statements = parser.parse();

            let printer = AstPrinter;
            for statement in statements {
                let output = printer.print_stmt(&statement);  // Print the AST in a readable format
                println!("{}", output);
            }
        },

        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

