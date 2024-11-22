use std::convert::From;
use std::fmt;
use std::rc::Rc;

use crate::compilation::token::TokenType;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Verb {
    None,
    Divide,
    Multiply,
    Subtract,
    Add,
    Assign,
    Action(Rc<str>),
}

impl fmt::Display for Verb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Verb::None => write!(f, "{:?}", self),
            Verb::Divide => write!(f, "/"),
            Verb::Multiply => write!(f, "*"),
            Verb::Subtract => write!(f, "-"),
            Verb::Add => write!(f, "+"),
            Verb::Assign => write!(f, "="),
            Verb::Action(verb) => write!(f, "{verb}"),
        }
    }
}

impl From<TokenType> for Verb {
    fn from(token: TokenType) -> Self {
        match token {
            TokenType::Slash => Verb::Divide,
            TokenType::Star => Verb::Multiply,
            TokenType::Minus => Verb::Subtract,
            TokenType::Plus => Verb::Add,
            TokenType::As => Verb::Assign,
            TokenType::Identifier => Verb::Action("<Action>".into()),
            _ => Verb::None,
        }
    }
}
