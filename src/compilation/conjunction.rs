use std::convert::From;
use std::fmt;

use crate::compilation::token::TokenType;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Conjunction {
    None,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
    Or,
    And,
}

impl fmt::Display for Conjunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Conjunction::None => write!(f, "{:?}", self),
            Conjunction::Greater => write!(f, ">"),
            Conjunction::GreaterEqual => write!(f, ">="),
            Conjunction::Less => write!(f, "<"),
            Conjunction::LessEqual => write!(f, "<="),
            Conjunction::Equal => write!(f, "=="),
            Conjunction::NotEqual => write!(f, "!="),
            Conjunction::Or => write!(f, "or"),
            Conjunction::And => write!(f, "and"),
        }
    }
}

impl From<TokenType> for Conjunction {
    fn from(token: TokenType) -> Self {
        match token {
            TokenType::Greater => Conjunction::Greater,
            TokenType::GreaterEqual => Conjunction::GreaterEqual,
            TokenType::Less => Conjunction::Less,
            TokenType::LessEqual => Conjunction::LessEqual,
            TokenType::Equal => Conjunction::Equal,
            TokenType::Tilde => Conjunction::NotEqual,
            TokenType::Or => Conjunction::Or,
            TokenType::And => Conjunction::And,
            _ => Conjunction::None,
        }
    }
}
