use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Left_Paren, Right_Paren, Left_Brace, Right_Brace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    Bang, Bang_Equal,
    Equal, Equal_Equal,
    Greater, Greater_Equal,
    Less, Less_Equal,

    Identifier, String, Number,

    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            TokenType::EOF => "EOF".to_string(),
            TokenType::Else => "ELSE".to_string(),
            TokenType::Number => "NUMBER".to_string(),
            TokenType::String => "STRING".to_string(),
            _ => format!("{:?}", self).to_uppercase() // No need to take a reference here, just use the String
        };
        write!(f, "{}", token_str)
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Str(String),
    Num(f64),
    Nil,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal_str = match &self.literal {
            Literal::Str(lit) => lit.clone(), 
            Literal::Num(num) => format_number_test!(*num), 
            Literal::Nil => "null".to_string(), 
        };
        
    
        write!(f, "{} {} {}", self.token_type, self.lexeme, literal_str)
    }
}

