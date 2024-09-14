use crate::parser::ParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("Operand must be a number.\n[line {line}]")]
    InvalidUnaryOperand { line: usize },

    #[error("Operands must be two numbers or two strings. [line {line}]")]
    InvalidBinaryOperands { line: usize },

    #[error("Undefined variable '{name}' at line {line}.")]
    UndefinedVariable { name: String, line: usize },

    #[error("Division by zero at line {line}.")]
    DivisionByZero { line: usize },

    #[error("Runtime Error: {message}")]
    GenericError { message: String },

    #[error("Parse error: {source}")]
    ParseError {
        #[from]
        source: ParseError,
    },
}
