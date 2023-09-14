use crate::{expr::{Expr, self}, scanner::{TokenType, Token, Literal}};
use crate::expr::BinaryOperator;
use crate::expr::BinaryOperatorEnum;

pub struct Parser {
    pub start: usize,
    pub current: usize,
    pub tokens: Vec<Token>
}

impl Parser {

    pub fn parse(&mut self) -> Expr {
        return self.expression().unwrap();
    }

    pub fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    pub fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.comparison().unwrap();

        while self.search(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison().unwrap();
            expr = Expr::Binary(Box::new(expr), Box::new(self.expr_from_tok(operator).unwrap()), Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr: Expr = self.term().unwrap();

        while self.search(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), Box::new(self.expr_from_tok(operator).unwrap()), Box::new(right.unwrap()));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor();

        while self.search(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Ok(Expr::Binary(Box::new(expr.unwrap()), Box::new(self.expr_from_tok(operator).unwrap()), Box::new(right.unwrap())));
        }
        expr
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary();

        while self.search(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Ok(Expr::Binary(Box::new(expr.unwrap()), Box::new(self.expr_from_tok(operator).unwrap()), Box::new(right.unwrap())));
        }
        expr
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.search(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Ok(Expr::Unary(Box::new(self.expr_from_tok(operator).unwrap()), Box::new(right.unwrap())));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.search(vec![TokenType::False]) {
            return Ok(Expr::Literal(Literal::False));
        }
        if self.search(vec![TokenType::True]) {
            return Ok(Expr::Literal(Literal::True));
        }
        if self.search(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }
        if self.search(vec![TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(self.previous().literal.unwrap()));
        }
        if self.search(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.".to_string()).unwrap();
            return Ok(Expr::Grouping(Box::new(expr.unwrap())));
        }

        Err(ParserError {
            line: None,
            lexme: None,
            message: "Expected expression.".to_string(),  
        })

        // Err(self.error(}, None, "Primary Expression Error"))
    }

    fn search(&mut self, vec: Vec<TokenType>) -> bool {
        for toktype in vec {
            if self.check(toktype) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn check(&self, toktype: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        let peek = self.peek().token_type;
        peek == toktype
    }

    fn advance(&mut self) -> Token{
        if !self.is_at_end() { self.current += 1; }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        let tok = self.tokens[self.current].clone();
        tok.clone()
    }

    fn consume(&mut self, tok: TokenType, err: String) -> Result<Token, ParserError>{
        dbg!("TOK: {}", tok);
        if self.check(tok) {
            return Ok(self.advance());
        }
        Err(self.error(self.peek(), err))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class => return,
                TokenType::Fun => return,
                TokenType::Var => return,
                TokenType::For => return,
                TokenType::If => return,
                TokenType::While => return,
                TokenType::Print => return,
                TokenType::Return => return,
                _ => continue,
            }
        }

        self.advance();
    }

    fn expr_from_tok(&self, operator: Token) -> Result<Expr, &'static str> {
        match operator.token_type {
            TokenType::EqualEqual => Ok(Expr::BinaryOp(BinaryOperator{
                token_type: BinaryOperatorEnum::EqualEqual,
                lexme: operator.lexme,
                literal: operator.literal,
                line: operator.line
            })),
            
            TokenType::BangEqual => Ok(Expr::BinaryOp(BinaryOperator{
                token_type: BinaryOperatorEnum::NotEqual,
                lexme: operator.lexme,
                literal: operator.literal,
                line: operator.line
            })),

            TokenType::Less => Ok(Expr::BinaryOp(BinaryOperator{
                token_type: BinaryOperatorEnum::Less,
                lexme: operator.lexme,
                literal: operator.literal,
                line: operator.line
            })),

            TokenType::LessEqual => Ok(Expr::BinaryOp(BinaryOperator{
                token_type: crate::expr::BinaryOperatorEnum::LessEqual,
                lexme: operator.lexme,
                literal: operator.literal,
                line: operator.line
            })),

            TokenType::Greater => Ok(Expr::BinaryOp(BinaryOperator{
                token_type: crate::expr::BinaryOperatorEnum::GreaterEqual,
                lexme: operator.lexme,
                literal: operator.literal,
                line: operator.line
            })),

            TokenType::GreaterEqual => Ok(Expr::BinaryOp(BinaryOperator{
                token_type: crate::expr::BinaryOperatorEnum::GreaterEqual,
                lexme: operator.lexme,
                literal: operator.literal,
                line: operator.line
            })),

            TokenType::Plus => Ok(Expr::BinaryOp(BinaryOperator{
                token_type: crate::expr::BinaryOperatorEnum::Plus,
                lexme: operator.lexme,
                literal: operator.literal,
                line: operator.line
            })),

            TokenType::Minus => Ok(Expr::BinaryOp(BinaryOperator{
                token_type: crate::expr::BinaryOperatorEnum::Minus,
                lexme: operator.lexme,
                literal: operator.literal,
                line: operator.line
            })),

            TokenType::Star => Ok(Expr::BinaryOp(BinaryOperator{
                token_type: crate::expr::BinaryOperatorEnum::Star,
                lexme: operator.lexme,
                literal: operator.literal,
                line: operator.line
            })),

            TokenType::Slash => Ok(Expr::BinaryOp(BinaryOperator{
                token_type: crate::expr::BinaryOperatorEnum::Slash,
                lexme: operator.lexme,
                literal: operator.literal,
                line: operator.line
            })),
            _ => {
                Err("ERROR")
            }
        }
    }

    fn error(&self, tok: Token, err: String) -> ParserError{
        if tok.token_type == TokenType::Eof {
            return ParserError{line: None, lexme: None, message: err};
        }
        return ParserError{line: Some(tok.line), lexme: Some(tok.lexme), message: err};
    }
}

#[derive(Debug)]
pub struct ParserError {
    line: Option<usize>,
    lexme: Option<String>,
    message: String,
}