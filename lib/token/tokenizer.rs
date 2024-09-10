use std::iter::Peekable;
use std::str::Chars;
use crate::token::token::{Token, TokenType};
use std::process;

pub fn tokenize(contents: &str) -> Vec<Token> {
    let mut chars = contents.chars().peekable();
    let mut line_number = 1;
    let mut tokens = Vec::new();
    let mut has_errors = false; 

    while let Some(&c) = chars.peek() {
        match c {
            '"' => parse_string(&mut chars, &mut tokens, line_number, &mut has_errors),
            '0'..='9' => parse_number(&mut chars, &mut tokens, line_number),
            '(' => push_simple_token(TokenType::Left_Paren, "(", &mut chars, &mut tokens, line_number),
            ')' => push_simple_token(TokenType::Right_Paren, ")", &mut chars, &mut tokens, line_number),
            '{' => push_simple_token(TokenType::Left_Brace, "{", &mut chars, &mut tokens, line_number),
            '}' => push_simple_token(TokenType::Right_Brace, "}", &mut chars, &mut tokens, line_number),
            '*' => push_simple_token(TokenType::Star, "*", &mut chars, &mut tokens, line_number),
            '.' => push_simple_token(TokenType::Dot, ".", &mut chars, &mut tokens, line_number),
            ',' => push_simple_token(TokenType::Comma, ",", &mut chars, &mut tokens, line_number),
            '-' => push_simple_token(TokenType::Minus, "-", &mut chars, &mut tokens, line_number),
            '+' => push_simple_token(TokenType::Plus, "+", &mut chars, &mut tokens, line_number),
            ';' => push_simple_token(TokenType::Semicolon, ";", &mut chars, &mut tokens, line_number),
            '=' => parse_double_char_token(&mut chars, &mut tokens, line_number, '=', TokenType::Equal, TokenType::Equal_Equal),
            '!' => parse_double_char_token(&mut chars, &mut tokens, line_number, '=', TokenType::Bang, TokenType::Bang_Equal),
            '<' => parse_double_char_token(&mut chars, &mut tokens, line_number, '=', TokenType::Less, TokenType::Less_Equal),
            '>' => parse_double_char_token(&mut chars, &mut tokens, line_number, '=', TokenType::Greater, TokenType::Greater_Equal),
            '/' => parse_slash(&mut chars, &mut tokens, line_number),
            ' ' | '\t' | '\r' => { chars.next(); } // Skip whitespace characters
            '\n' => { line_number += 1; chars.next(); } // Newline handling
            _ if is_alpha(c) => parse_identifier(&mut chars, &mut tokens, line_number),
            _ => {
                // Handle unexpected character
                eprintln!("[line {}] Error: Unexpected character: {}", line_number, c);
                has_errors = true;
                chars.next();
            }
        }
    }

    // Add EOF token at the end
    tokens.push(Token::new(TokenType::EOF, String::from(""), None, line_number));

    // Print all tokens to stdout
    for token in &tokens {
        println!("{}", token);
    }

    // If there were any errors, exit with code 65
    if has_errors {
        process::exit(65);
    }

    tokens
}

fn push_simple_token(token_type: TokenType, lexeme: &str, chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>, line: usize) {
    chars.next(); // consume the character
    tokens.push(Token::new(token_type, String::from(lexeme), None, line));
}

fn parse_double_char_token(
    chars: &mut Peekable<Chars>,
    tokens: &mut Vec<Token>,
    line: usize,
    expected_next: char,
    single_char_type: TokenType,
    double_char_type: TokenType,
) {
    let mut lexeme = String::from(chars.next().unwrap());
    if chars.peek() == Some(&expected_next) {
        lexeme.push(chars.next().unwrap());
        tokens.push(Token::new(double_char_type, lexeme, None, line));
    } else {
        tokens.push(Token::new(single_char_type, lexeme, None, line));
    }
}

fn parse_slash(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>, line: usize) {
    chars.next(); // consume '/'
    if chars.peek() == Some(&'/') {
        // Comment, consume until end of line
        while let Some(&next_char) = chars.peek() {
            if next_char == '\n' { break; }
            chars.next();
        }
    } else {
        tokens.push(Token::new(TokenType::Slash, String::from("/"), None, line));
    }
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || c.is_ascii_digit()
}

fn parse_identifier(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>, line: usize) {
    let mut identifier = String::new();
    while let Some(&c) = chars.peek() {
        if is_alphanumeric(c) {
            identifier.push(c);
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

    tokens.push(Token::new(token_type, identifier, None, line));
}
fn parse_string(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>, line: usize, has_errors: &mut bool) {
    let mut string_content = String::new();
   chars.next();

    while let Some(&c) = chars.peek() {
        if c == '"' {
            chars.next(); 
            let lexeme = format!("\"{}\"", string_content); 
            let literal = string_content.clone();
            tokens.push(Token::new(TokenType::String, lexeme, Some(literal), line));
            return; 
        }
        if c == '\n' || chars.peek().is_none() {
            eprintln!("[line {}] Error: Unterminated string.", line);
            *has_errors = true;
            return;
        }
        string_content.push(c);
        chars.next(); 
    }

    eprintln!("[line {}] Error: Unterminated string.", line);
    *has_errors = true;
}

fn parse_number(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>, line: usize) {
    let mut number = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() || c == '.' {
            number.push(c);
            chars.next();
        } else {
            break;
        }
    }

    tokens.push(Token::new(TokenType::Number, number.clone(), Some(number), line));
}
