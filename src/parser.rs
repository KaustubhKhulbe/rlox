use crate::{expr::{Expr, self}, scanner::{TokenType, Token, Literal}};
use crate::expr::BinaryOperator;
use crate::expr::BinaryOperatorEnum;

pub struct Parser {
    start: usize,
    current: usize,
    tokens: Vec<Token>
}

impl Parser {
    pub fn expression(&mut self) -> Expr {
        self.equality()
    }

    pub fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.search(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::Binary(Box::new(expr), Box::new(self.expr_from_tok(operator).unwrap()), Box::new(right));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while self.search(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), Box::new(self.expr_from_tok(operator).unwrap()), Box::new(right));
        }

        expr
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

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.search(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), Box::new(self.expr_from_tok(operator).unwrap()), Box::new(right));
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.search(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), Box::new(self.expr_from_tok(operator).unwrap()), Box::new(right));
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.search(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary(Box::new(self.expr_from_tok(operator).unwrap()), Box::new(right));
        }

        self.primary().unwrap()
    }

    fn primary(&mut self) -> Result<Expr, &str> {
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
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err("Primary Call Error")
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn check(&self, toktype: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == toktype
    }

    fn advance(&mut self) -> Token{
        if !self.is_at_end() { self.current += 1; }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn consume(&mut self, tok: TokenType, err: String) -> Result<Token, &str>{
        if self.check(tok) {
            return Ok(self.advance());
        }
        // Err(self.error(self.peek(), err))
        Err(self.error(self.peek(), err))
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

    fn error(&self, tok: Token, err: String) -> &str {
        if tok.token_type == TokenType::Eof {
            return format!("{} at end", err).as_str();
        }
        format!("{} at '{}' {}", tok.line, tok.lexme, err).as_str()

    }
}