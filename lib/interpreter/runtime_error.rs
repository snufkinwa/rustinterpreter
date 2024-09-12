use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("Operand must be a number.")]
    InvalidUnaryOperand { line: usize },
    
    #[error("Operands must be two numbers or two strings.")]
    InvalidBinaryOperands { line: usize },

}
