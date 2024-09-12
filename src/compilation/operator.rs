use std::convert::From;
use std::fmt;

use crate::compilation::token::TokenType;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Operator {
    None,
    Not,
    Negation,
    Division,
    Multiplication,
    Subtraction,
    Addition,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
    Assignment,
    Or,
    And,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::None => write!(f, "{:?}", self),
            Operator::Not => write!(f, "!"),
            Operator::Negation => write!(f, "-"),
            Operator::Division => write!(f, "/"),
            Operator::Multiplication => write!(f, "*"),
            Operator::Subtraction => write!(f, "-"),
            Operator::Addition => write!(f, "+"),
            Operator::Greater => write!(f, ">"),
            Operator::GreaterEqual => write!(f, ">="),
            Operator::Less => write!(f, "<"),
            Operator::LessEqual => write!(f, "<="),
            Operator::Equal => write!(f, "=="),
            Operator::NotEqual => write!(f, "!="),
            Operator::Assignment => write!(f, "="),
            Operator::Or => write!(f, "or"),
            Operator::And => write!(f, "and"),
        }
    }
}

impl From<TokenType> for Operator {
    fn from(token: TokenType) -> Self {
        match token {
            TokenType::Bang => Operator::Not,
            TokenType::Slash => Operator::Division,
            TokenType::Star => Operator::Multiplication,
            TokenType::Minus => Operator::Subtraction,
            TokenType::Plus => Operator::Addition,
            TokenType::Greater => Operator::Greater,
            TokenType::GreaterEqual => Operator::GreaterEqual,
            TokenType::Less => Operator::Less,
            TokenType::LessEqual => Operator::LessEqual,
            TokenType::Equal => Operator::Equal,
            TokenType::Tilde => Operator::NotEqual,
            TokenType::As => Operator::Assignment,
            TokenType::Or => Operator::Or,
            TokenType::And => Operator::And,
            _ => Operator::None,
        }
    }
}
