use std::fmt::{Display, self};

use crate::scanner::{Literal, TokenType};

pub trait Visitor {
    // fn visit_binary_expr(expr: Expr) -> String;
    fn accept(&self, visitor: Box<dyn Visitor>);
}
pub enum Expr {
    Literal(Literal),
    Unary(Box<Expr>, Box<Expr>),
    Binary(Box<Expr>, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
    BinaryOp(BinaryOperator),
    UnaryOp(UnaryOperator)
}

impl Expr {
    pub fn visit(expr: Expr) -> String {
        match expr {
            Expr::Binary(left, operator, right) => Expr::parenthesize("Binary Operation".to_string(), vec![*left, *operator, *right]),
            Expr::Unary(operator, expression) => Expr::parenthesize("Unary Operation".to_string(), vec![*operator, *expression]),
            Expr::Grouping(group) => Expr::parenthesize("Grouping Operation".to_string(), vec![*group]),
            Expr::Literal(literal) => format!("{}", literal),
            Expr::BinaryOp(operator) => operator.token_type.to_string(),
            Expr::UnaryOp(operator) => operator.token_type.to_string(),
            _ => "".to_string()
        }
    }

    fn parenthesize(name: String, exprs: Vec<Expr>) -> String{
        let mut res = "(".to_string() + &name;
        for expr in exprs {
            let s = Expr::visit(expr);
            res = res + &s.to_string() + "";
        }
        res + ")"
    }
}

impl Visitor for Expr {
    fn accept(&self, visitor: Box<dyn Visitor>) {
        todo!()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BinaryOperatorEnum {
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}

impl Display for BinaryOperatorEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EqualEqual => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
            Self::Less => write!(f, "<"),
            Self::LessEqual => write!(f, "<="),
            Self::Greater => write!(f, ">"),
            Self::GreaterEqual => write!(f, ">="),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
        }
    }
}

pub struct BinaryOperator {
    pub token_type: BinaryOperatorEnum,
    pub lexme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

pub enum UnaryOperatorEnum {
    Minus,
    Bang
}

pub struct UnaryOperator {
    pub token_type: UnaryOperatorEnum,
    pub lexme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Display for UnaryOperatorEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minus => write!(f, "-"),
            Self::Bang => write!(f, "!"),
        }
    }
}