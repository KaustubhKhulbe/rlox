use core::fmt;
use std::any::Any;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum TokenType{
    // single character tokens
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE, 
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // one or two character tokens
    BANG, BANG_EQUAL, EQUAL, EQUAL_EQUAL, 
    GREATER, GREATER_EQUAL, LESS, LESS_EQUAL,

    // literals
    IDENTIFIER, STRING, NUMBER,

    // keywords
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexme: String,
    pub literal: Box<dyn Any>,
    pub line: usize,
}

impl Token {
    pub fn to_string(&self) -> String{
        let s = self.token_type.to_string() + " " + &self.lexme + " " + &self.line.to_string();
        s.to_string()
    }
}