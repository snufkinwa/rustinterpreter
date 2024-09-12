use crate::token::token::Token;
use crate::token::tokenizer_error::TokenizerError;

pub fn print_tokens_and_errors(tokens: &Vec<Token>, errors: &Vec<TokenizerError>) {
    if !errors.is_empty() {
        for error in errors {
            eprintln!("{}", error);
        }
    }

    for token in tokens {
        println!("{}", token);
    }
}
