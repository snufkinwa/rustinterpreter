<p align="center">
  <img src="https://imgur.com/NYvUP9v.gif" />
</p>

# Rust Interpreter Project - Codecrafters Challenge

This project is a Rust-based interpreter for the ["Build Your Own Interpreter" Challenge](https://app.codecrafters.io/courses/interpreter/overview) from Codecrafters, following the structure and concepts presented in the book [Crafting Interpreters](https://craftinginterpreters.com/) by Robert Nystrom.

## Overview

In this challenge, the goal is to build an interpreter for the [Lox language](https://craftinginterpreters.com/the-lox-language.html), a simple scripting language. The project helps solidify core concepts of interpreters, including tokenization, abstract syntax trees (ASTs), and tree-walk interpreters.

## Key Components

- **Tokenization**: Handles the process of breaking down input strings into meaningful symbols (tokens) used by the interpreter.
- **Parsing**: Constructs an Abstract Syntax Tree (AST) from the tokens, forming the structure of the code for interpretation.
- **AST Printer**: Outputs the structure of parsed expressions for debugging and testing.
- **Scanning**: Converts characters into tokens, following the Lox language grammar.
- **Tree-Walk Interpreter** (Coming Soon): Future work will include a tree-walk interpreter that directly interprets the Abstract Syntax Tree (AST) and executes the code.

## How to Run

1. **Install dependencies**: This project uses Rust. Ensure that you have Rust installed by following the instructions [here](https://www.rust-lang.org/tools/install).

2. **Tokenize a Lox file**: To tokenize your input Lox code:
```bash
   ./your_program.sh tokenize tests/test.lox
```
  Parse a Lox file: To parse your input Lox code and generate the AST:
  
  ```bash
    ./your_program.sh parse tests/test.lox
   ```
    
## Development

This project is organized into several core modules:

- token: Responsible for handling the different types of tokens in Lox.
- scanner: Scans through the source code and tokenizes it.
- parser: Builds the Abstract Syntax Tree (AST) from tokens.
- ast_printer: A utility for visualizing the AST.

## Future Work

- A tree-walk interpreter will be added, allowing direct interpretation and execution of the AST.
- Support for functions, classes, and additional language features may be implemented to extend the Lox interpreter.

## References

[Crafting Interpreters](https://craftinginterpreters.com/)

[Lox Language](https://craftinginterpreters.com/the-lox-language.html)

## Progress

You can track my progress on the Codecrafters platform [here](https://app.codecrafters.io/users/snufkinwa).
