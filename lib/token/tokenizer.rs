use bytes::Bytes;
use std::iter::Peekable;
use crate::token::token::{Literal, Token, TokenType};
use std::slice::Iter;
use anyhow::Result;
use super::tokenizer_error::TokenizerError;

// Main tokenize function
pub fn tokenize(contents: Bytes) ->Result<( Vec<Token>, Vec<TokenizerError>)> {
    let mut chars = contents.iter().peekable(); // Use an iterator over bytes
    let mut line_number = 1;
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    while let Some(&c) = chars.peek() {
        match c {
            b'"' => {
                match parse_string(&mut chars, &mut tokens, line_number) {
                    Ok(()) => {}, 
                    Err(err) => {
                        errors.push(err);
                    }
                }
            }
            b'0'..=b'9' => parse_number(&mut chars, &mut tokens, line_number),
            b'(' => push_simple_token(TokenType::Left_Paren, "(", &mut chars, &mut tokens, line_number),
            b')' => push_simple_token(TokenType::Right_Paren, ")", &mut chars, &mut tokens, line_number),
            b'{' => push_simple_token(TokenType::Left_Brace, "{", &mut chars, &mut tokens, line_number),
            b'}' => push_simple_token(TokenType::Right_Brace, "}", &mut chars, &mut tokens, line_number),
            b'*' => push_simple_token(TokenType::Star, "*", &mut chars, &mut tokens, line_number),
            b'.' => push_simple_token(TokenType::Dot, ".", &mut chars, &mut tokens, line_number),
            b',' => push_simple_token(TokenType::Comma, ",", &mut chars, &mut tokens, line_number),
            b'-' => push_simple_token(TokenType::Minus, "-", &mut chars, &mut tokens, line_number),
            b'+' => push_simple_token(TokenType::Plus, "+", &mut chars, &mut tokens, line_number),
            b';' => push_simple_token(TokenType::Semicolon, ";", &mut chars, &mut tokens, line_number),
            b'=' => parse_double_char_token(&mut chars, &mut tokens, line_number, b'=', TokenType::Equal, TokenType::Equal_Equal),
            b'!' => parse_double_char_token(&mut chars, &mut tokens, line_number, b'=', TokenType::Bang, TokenType::Bang_Equal),
            b'<' => parse_double_char_token(&mut chars, &mut tokens, line_number, b'=', TokenType::Less, TokenType::Less_Equal),
            b'>' => parse_double_char_token(&mut chars, &mut tokens, line_number, b'=', TokenType::Greater, TokenType::Greater_Equal),
            b'/' => parse_slash(&mut chars, &mut tokens, line_number),
            b' ' | b'\t' | b'\r' => { chars.next(); } // Ignore whitespace
            b'\n' => { line_number += 1; chars.next(); } // Handle new lines
            _ if is_alpha(c) => parse_identifier(&mut chars, &mut tokens, line_number),
            _ => {
                errors.push(TokenizerError::UnexpectedCharacter {
                    line: line_number,
                    ch: *c as char,
                });
                chars.next();
            }
        }
    }

    // Push EOF token after processing all characters
    tokens.push(Token::new(TokenType::EOF, String::new(), Literal::Nil, line_number));
    
    Ok((tokens, errors))
}

fn push_simple_token(token_type: TokenType, lexeme: &str, chars: &mut Peekable<std::slice::Iter<u8>>, tokens: &mut Vec<Token>, line: usize) {
    chars.next();  // Consume the byte
    tokens.push(Token::new(token_type, lexeme.to_string(), Literal::Nil, line));
}

fn parse_double_char_token(
    chars: &mut Peekable<std::slice::Iter<u8>>,
    tokens: &mut Vec<Token>,
    line: usize,
    expected_next: u8,
    single_char_type: TokenType,
    double_char_type: TokenType,
) {
    let mut lexeme = String::from_utf8(vec![*chars.next().unwrap()]).unwrap();  // Get the first byte and convert to string
    if chars.peek() == Some(&&expected_next) {
        lexeme.push(*chars.next().unwrap() as char);  // Append the second byte
        tokens.push(Token::new(double_char_type, lexeme, Literal::Nil, line));
    } else {
        tokens.push(Token::new(single_char_type, lexeme, Literal::Nil, line));
    }
}

fn parse_slash(chars: &mut Peekable<std::slice::Iter<u8>>, tokens: &mut Vec<Token>, line: usize) {
    chars.next();  // Consume '/'
    if chars.peek() == Some(&&b'/') {
        // It's a comment, consume until end of line
        while let Some(&&next_char) = chars.peek() {
            if next_char == b'\n' { break; }
            chars.next();
        }
    } else {
        tokens.push(Token::new(TokenType::Slash, "/".to_string(), Literal::Nil, line));
    }
}

fn is_alpha(c: &u8) -> bool {
    c.is_ascii_alphabetic() || *c == b'_'
}

fn is_alphanumeric(c: &u8) -> bool {
    is_alpha(c) || c.is_ascii_digit()
}

fn parse_identifier(chars: &mut Peekable<std::slice::Iter<u8>>, tokens: &mut Vec<Token>, line: usize) {
    let mut identifier = String::new();
    while let Some(&c) = chars.peek() {
        if is_alphanumeric(&c) {
            identifier.push(*c as char);
            chars.next();
        } else {
            break;
        }
    }

    let token_type = match identifier.as_str() {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "for" => TokenType::For,
        "fun" => TokenType::Fun,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        _ => TokenType::Identifier,
    };

    tokens.push(Token::new(token_type, identifier, Literal::Nil, line));
}

fn parse_string(
    chars: &mut Peekable<Iter<u8>>, 
    tokens: &mut Vec<Token>, 
    mut line: usize
) -> Result<(), TokenizerError> {
    let mut utf8_bytes = Vec::new();  // To collect the bytes for the string content
    chars.next(); // Consume the opening quote 

    while let Some(&c) = chars.peek() {
        if c == &b'"' {
            chars.next(); // Consume the closing quote

            // Convert the collected bytes to a valid UTF-8 string
            let string_content = match String::from_utf8(utf8_bytes) {
                Ok(s) => s,
                Err(_) => return Err(TokenizerError::InvalidUTF8String { line }),
            };

            tokens.push(Token::new(
                TokenType::String,
                format!("\"{}\"", string_content),
                Literal::Str(string_content),
                line,
            ));
            return Ok(()); // Successfully parsed string
        }

        if c == &b'\n' {
            // Allow newlines inside strings, increment line number
            utf8_bytes.push(b'\n');  // Push newline to the byte buffer
            line += 1;
            chars.next();
            continue;
        }

        if chars.peek().is_none() {
            // If we run out of characters, it's an unterminated string
            return Err(TokenizerError::UnterminatedString { line });
        }

        // Collect the byte for the string content
        utf8_bytes.push(*c);
        chars.next(); 
    }

    // If we reach here, it means the string wasn't properly closed (unterminated)
    Err(TokenizerError::UnterminatedString { line })
}

fn parse_number(chars: &mut Peekable<std::slice::Iter<u8>>, tokens: &mut Vec<Token>, line: usize) {
    let mut value: f64 = 0.0;
    let mut divisor: f64 = 1.0;
    let mut is_fractional = false;
    let mut lexeme = String::new();  // Capture the lexeme as it appears in the source

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            lexeme.push(*c as char);  // Append the current character to the lexeme
            let digit = (c - b'0') as f64;
            if is_fractional {
                divisor *= 10.0;
                value += digit / divisor;
            } else {
                value = value * 10.0 + digit;
            }
            chars.next();  // Move to the next character
        } else if c == &b'.' {
            if is_fractional {
                break;  // Only allow one decimal point
            }
            is_fractional = true;
            lexeme.push(*c as char);  // Append the decimal point to the lexeme
            chars.next();
        } else {
            break;
        }
    }

    // Push the token with the exact lexeme and literal value
    tokens.push(Token::new(TokenType::Number, lexeme, Literal::Num(value), line));
}


