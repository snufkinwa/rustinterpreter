use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenizerError {
    #[error("[line {line}] Error: Unterminated string.")]
    UnterminatedString { line: usize },
    
    #[error("[line {line}] Error: Unexpected character: {ch}")]
    UnexpectedCharacter { line: usize, ch: char },

    #[error("[line {line}] Error: Invalid UTF-8 sequence in string.")]
    InvalidUTF8String { line: usize },
}
