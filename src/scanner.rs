use std::{fmt::{Debug, Display, self}, collections::HashMap};

use crate::error;


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TokenType {
    // single character tokens
    LeftParen, RightParen, LeftBrace, RightBrace, 
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // one or two character tokens
    Bang, BangEqual, Equal, EqualEqual, 
    Greater, GreaterEqual, Less, LessEqual,

    // literals
    Identifier, String, Number,

    // keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Str(String),
    Num(f64),
    False,
    True,
    Nil
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Str(s) => write!(f, "{}", s),
            Self::Num(n) => write!(f, "{}", n),
            s => write!(f, "{}", s)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn to_string(&self) -> String {
        let s = 
        "tok: ".to_string() + &self.token_type.to_string() + 
        " lex: " + &self.lexme + 
        " line: " + &self.line.to_string();
        s.to_string()
    }
}

pub struct Scanner {
    source: String,
    source_chars: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    pub current: usize,
    pub line: usize,
    map: HashMap<String, TokenType>
}

impl Default for Scanner {
    fn default() -> Self {
        let mut m = HashMap::new();
        m.insert("and".to_string(), TokenType::And);
        m.insert("class".to_string(), TokenType::Class);
        m.insert("else".to_string(), TokenType::Else);
        m.insert("false".to_string(), TokenType::False);
        m.insert("for".to_string(), TokenType::For);
        m.insert("fun".to_string(), TokenType::Fun);
        m.insert("if".to_string(), TokenType::If);
        m.insert("nil".to_string(), TokenType::Nil);
        m.insert("or".to_string(), TokenType::Or);
        m.insert("print".to_string(), TokenType::Print);
        m.insert("return".to_string(), TokenType::Return);
        m.insert("super".to_string(), TokenType::Super);
        m.insert("this".to_string(), TokenType::This);
        m.insert("true".to_string(), TokenType::True);
        m.insert("var".to_string(), TokenType::Var);
        m.insert("while".to_string(), TokenType::While);

        Self { 
            source: Default::default(), 
            source_chars: Default::default(), 
            tokens: Default::default(), 
            start: Default::default(), 
            current: Default::default(), 
            line: Default::default(), 
            map: m 
        }
    }
}

impl Scanner {
    pub fn scan_tokens(&mut self, source: String) -> &Vec<Token> {
        self.source = source.clone();
        self.source_chars = source.as_str().chars().collect();
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let eof = Token {
            token_type: TokenType::Eof,
            lexme: "".to_string(),
            literal: None,
            line: self.line
        };

        self.tokens.push(eof);
        &self.tokens
    }

    fn is_at_end(&self) -> bool{
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        self.match_toks(c);
    }

    pub fn match_toks(&mut self, c: char) {
        match c {
            '(' => self.add_token2(TokenType::LeftParen),
            ')' => self.add_token2(TokenType::RightParen),
            '{' => self.add_token2(TokenType::LeftBrace),
            '}' => self.add_token2(TokenType::RightBrace),
            ',' => self.add_token2(TokenType::Comma),
            '.' => self.add_token2(TokenType::Dot),
            '-' => self.add_token2(TokenType::Minus),
            '+' => self.add_token2(TokenType::Plus),
            '*' => self.add_token2(TokenType::Star),
            ';' => self.add_token2(TokenType::Semicolon),
            '!' => if self.matched('=') { self.add_token2(TokenType::BangEqual)} else {self.add_token2(TokenType::Bang)},
            '=' => if self.matched('=') { self.add_token2(TokenType::EqualEqual)} else {self.add_token2(TokenType::Equal)},
            '<' => if self.matched('=') { self.add_token2(TokenType::LessEqual)} else {self.add_token2(TokenType::Less)},
            '>' => if self.matched('=') { self.add_token2(TokenType::GreaterEqual)} else {self.add_token2(TokenType::Greater)},
            '/' => { // todo(CHECK why unreachable pattern)
                if self.matched('/') {
                    while self.peek() != '\n' && !self.is_at_end() {self.advance();}
                } else {self.add_token2(TokenType::Slash);}
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

    fn add_token(&mut self, token: TokenType, token_type: Option<Literal>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token { 
            token_type: token, 
            lexme: text.to_string(), 
            literal: token_type, 
            line: self.line 
        });
    }

    fn add_token2(&mut self, token: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token { 
            token_type: token, 
            lexme: text.to_string(), 
            literal: None, 
            line: self.line 
        })
    }

    fn matched(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }

        if self.source_chars[self.current] != expected {
            self.current += 1;
            return false ;   
        }
        true
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) { self.advance(); }
        let text = &self.source[self.start..self.current];
        let t = *match self.map.get(text) {
            Some(x) => x,
            None => &TokenType::Identifier,
        };

        self.add_token(t, None);
    }

    fn string(&mut self) {
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
        self.add_token(TokenType::String, Some(Literal::Str(value)));
    }

    fn number(&mut self) {
        while (self.peek().is_numeric() || self.peek() == '.') && !self.is_at_end() {
            self.advance();
        }
        
        let value = self.source[self.start..self.current].to_string();
        self.add_token(TokenType::Number, Some(Literal::Num(value.parse::<f64>().unwrap())));
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0'; }
        self.source_chars[self.current]
    }

    fn advance(&mut self) -> char {
        let c = self.source_chars[self.current];
        self.current += 1;
        c
    }

    fn is_alpha(&mut self, c: char) -> bool {
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
    }

    fn is_alpha_numeric(&mut self, c: char) -> bool {
        self.is_alpha(c) || c.is_numeric()
    }

}
