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
            '/' => self.add_token2(TokenType::SLASH),
            '*' => self.add_token2(TokenType::STAR),
            '!' => if self.matched('=') { self.add_token2(TokenType::BANG_EQUAL)} else {self.add_token2(TokenType::BANG)},
            '=' => if self.matched('=') { self.add_token2(TokenType::EQUAL_EQUAL)} else {self.add_token2(TokenType::EQUAL)},
            '<' => if self.matched('=') { self.add_token2(TokenType::LESS_EQUAL)} else {self.add_token2(TokenType::LESS)},
            '>' => if self.matched('=') { self.add_token2(TokenType::GREATER_EQUAL)} else {self.add_token2(TokenType::GREATER)},
            '/' => todo!("implement slash"),
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
        self.current+= 1;
        self.source_chars[self.current]
    }

    fn matched(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source_chars[self.current] != expected { return false; }
        self.current+= 1;
        true
    }
}