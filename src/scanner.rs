use std::{any::Any, fmt::Debug, collections::HashMap, sync::{Mutex, MutexGuard}};

use crate::{token::{Token, TokenType}, error};
use lazy_static::lazy_static;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<&'static str, TokenType>> = {
        let mut m = HashMap::new();
        m.insert("and", TokenType::AND);
        m.insert("class", TokenType::CLASS);
        m.insert("else", TokenType::ELSE);
        m.insert("false", TokenType::FALSE);
        m.insert("for", TokenType::FOR);
        m.insert("fun", TokenType::FUN);
        m.insert("if", TokenType::IF);
        m.insert("nil", TokenType::NIL);
        m.insert("or", TokenType::OR);
        m.insert("print", TokenType::PRINT);
        m.insert("return", TokenType::RETURN);
        m.insert("super", TokenType::SUPER);
        m.insert("this", TokenType::THIS);
        m.insert("true", TokenType::TRUE);
        m.insert("var", TokenType::VAR);
        m.insert("while", TokenType::WHILE);

        Mutex::new(m)
    };    
}

pub trait Literal : Debug {
    fn as_any(&self) -> &dyn Any;
}

impl Literal for f64 {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Literal for String {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Scanner<'a> {
    pub source: String,
    pub source_chars: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    map: MutexGuard<'a, HashMap<&'static str, TokenType>>,
}

impl Scanner<'_> {
    pub fn new(source: String) -> Scanner<'static> {
        let text = source.as_str();

            Scanner{
            source: text.to_string(),
            source_chars: text.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            map: HASHMAP.lock().unwrap()
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token>{
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let eof = Token {
            token_type: TokenType::EOF,
            lexme: "".to_string(),
            literal: Box::new(0.0),
            line: self.line,
        };

        self.tokens.push(eof);

        &self.tokens
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token2(TokenType::LEFT_PAREN),
            ')' => self.add_token2(TokenType::RIGHT_PAREN),
            '{' => self.add_token2(TokenType::LEFT_BRACE),
            '}' => self.add_token2(TokenType::RIGHT_BRACE),
            ',' => self.add_token2(TokenType::COMMA),
            '.' => self.add_token2(TokenType::DOT),
            '-' => self.add_token2(TokenType::MINUS),
            '+' => self.add_token2(TokenType::PLUS),
            '*' => self.add_token2(TokenType::STAR),
            ';' => self.add_token2(TokenType::SEMICOLON),
            '!' => if self.matched('=') { self.add_token2(TokenType::BANG_EQUAL)} else {self.add_token2(TokenType::BANG)},
            '=' => if self.matched('=') { self.add_token2(TokenType::EQUAL_EQUAL)} else {self.add_token2(TokenType::EQUAL)},
            '<' => if self.matched('=') { self.add_token2(TokenType::LESS_EQUAL)} else {self.add_token2(TokenType::LESS)},
            '>' => if self.matched('=') { self.add_token2(TokenType::GREATER_EQUAL)} else {self.add_token2(TokenType::GREATER)},
            '/' => { // todo(CHECK why unreachable pattern)
                if self.matched('/') {
                    while self.peek() != '\n' && !self.is_at_end() {self.advance();}
                } else {self.add_token2(TokenType::SLASH);}
            },
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            '"' => {self.string()},

            _ => {
                if c.is_numeric() {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                }
                else {
                    error(self.line.try_into().unwrap(), "Unexpected character.")
                }
            }
        }
    }

    pub fn add_token(&mut self, token: TokenType, token_type: Box<dyn Literal>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token { token_type: token, lexme: text.to_string(), literal: token_type, line: self.line })
    }

    pub fn add_token2(&mut self, token: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token { token_type: token, lexme: text.to_string(), literal: Box::new(0.0), line: self.line })
    }

    pub fn advance(&mut self) -> char{
        let c = self.source_chars[self.current];
        self.current+= 1;
        c
    }

    fn matched(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source_chars[self.current] != expected { return false; }
        self.current+= 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0' }
        self.source_chars[self.current]
    }

    pub fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line+= 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line.try_into().unwrap(), "Unterminated string.");
        }

        self.advance();

        let value: String = self.source[self.start+1..self.current-1].to_string();
        dbg!(&value);
        self.add_token(TokenType::STRING, Box::new(value));
    }

    fn number(&mut self) {
        while (self.peek().is_numeric() || self.peek() == '.') && !self.is_at_end() {
            self.advance();
        }
        
        let value = self.source[self.start..self.current].to_string();
        self.add_token(TokenType::NUMBER, Box::new(value.parse::<f64>().unwrap()));
    }

    #[allow(dead_code)]
    pub fn extract_float(tok: &Token) -> f64{
        *match tok.literal.as_any().downcast_ref::<f64>() {
            Some(f) => f,
            None => panic!("Token Literal is not f64"),
        }
    }

    #[allow(dead_code)]
    pub fn extract_str(tok: &Token) -> String{
        match tok.literal.as_any().downcast_ref::<String>() {
            Some(str) => str.to_string(),
            None => panic!("Token Literal is not String"),
        }
    }

    fn is_alpha(&mut self, c: char) -> bool {
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
    }

    fn is_alpha_numeric(&mut self, c: char) -> bool {
        self.is_alpha(c) || c.is_numeric()
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) { self.advance(); }
        let text = &self.source[self.start..self.current];
        let t = *match self.map.get(text) {
            Some(x) => x,
            None => &TokenType::IDENTIFIER,
        };

        self.add_token(t, Box::new(0.0));
    }
}

#[cfg(test)]
mod tests {
    use crate::{scanner::Scanner, token::TokenType};

    #[test]
    fn scanner_new() {
        let s = Scanner::new("Hello World".to_string());
        assert_eq!(s.source, "Hello World");

        let chars: Vec<char> = ['H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd'].to_vec();
        assert_eq!(s.source_chars, chars);
    }

    #[test]
    fn scanner_add_tok() {
        let mut s = Scanner::new("Hello World".to_string());
        s.add_token2(crate::token::TokenType::AND);
        assert_eq!(s.tokens[0].token_type, crate::token::TokenType::AND);

        s.add_token2(crate::token::TokenType::SLASH);
        assert_eq!(s.tokens[1].token_type, crate::token::TokenType::SLASH);
    }

    #[test]
    fn scanner_advance() {
        let mut s = Scanner::new("Hello World".to_string());
        assert_eq!(s.current, 0);
        s.advance();
        assert_eq!(s.current, 1);
    }

    #[test]
    fn scanner_is_at_end() {
        let mut s = Scanner::new("ABC".to_string());
        assert_eq!(s.is_at_end(), false);
        s.advance();
        s.advance();
        s.advance();
        dbg!(s.current);
        assert_eq!(s.is_at_end(), true);
    }

    #[test]
    fn scanner_scan_tok() {
        let mut s = Scanner::new("()!=//".to_string());
        s.scan_token();
        assert_eq!(s.tokens[0].token_type, crate::token::TokenType::LEFT_PAREN);
        s.scan_token();
        assert_eq!(s.tokens[1].token_type, crate::token::TokenType::RIGHT_PAREN);
        s.scan_token();
        assert_eq!(s.tokens[2].token_type, crate::token::TokenType::BANG_EQUAL);
        s.scan_token();
        assert_eq!(s.is_at_end(), true);
    }

    #[test]
    fn scanner_scan_toks() {
        let mut s = Scanner::new("(({) / // \n { \"Hello World\" 3.14".to_string());
        s.scan_tokens();
        let toks = [TokenType::LEFT_PAREN, TokenType::LEFT_PAREN, TokenType::LEFT_BRACE, TokenType::RIGHT_PAREN, TokenType::SLASH, TokenType::LEFT_BRACE, TokenType::STRING, TokenType::NUMBER, TokenType::EOF];
        for i in 0..s.tokens.len() {
            assert_eq!(s.tokens[i].token_type, toks[i]);
        }

        dbg!(&s.tokens[s.tokens.len()-1].to_string());
    }
}