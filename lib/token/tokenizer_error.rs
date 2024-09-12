use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenizerError {
    #[error("[line {line}] Error: Unterminated string.")]
    UnterminatedString { line: usize },
    
    #[error("[line {line}] Error: Unexpected character: {ch}")]
    UnexpectedCharacter { line: usize, ch: char },
}
