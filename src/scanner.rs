use std::any::Any;

use crate::{token::{Token, TokenType}, error};

pub struct Scanner {
    pub source: String,
    pub source_chars: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let text = source.as_str();

            Scanner{
            source: text.to_string(),
            source_chars: text.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
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
            literal: Box::new({}),
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

            _ => error(self.line.try_into().unwrap(), "Unexpected character.")
        }
    }

    pub fn add_token(&mut self, token: TokenType, token_type: Box<dyn Any>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token { token_type: token, lexme: text.to_string(), literal: token_type, line: self.line })
    }

    pub fn add_token2(&mut self, token: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token { token_type: token, lexme: text.to_string(), literal: Box::new({}), line: self.line })
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
        let mut s = Scanner::new("(({) / // \n {".to_string());
        s.scan_tokens();
        let toks = [TokenType::LEFT_PAREN, TokenType::LEFT_PAREN, TokenType::LEFT_BRACE, TokenType::RIGHT_PAREN, TokenType::SLASH, TokenType::LEFT_BRACE, TokenType::EOF];
        for i in 0..s.tokens.len() {
            assert_eq!(s.tokens[i].token_type, toks[i]);
        }
    }
}