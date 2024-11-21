use std::convert::From;
use std::fmt;
use std::rc::Rc;

use crate::compilation::token::{Token, TokenType};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Prefix {
    None,
    Not,
    Negation,
    Adjective(Rc<str>),
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Prefix::None => write!(f, "{:?}", self),
            Prefix::Not => write!(f, "!"),
            Prefix::Negation => write!(f, "-"),
            Prefix::Adjective(adj) => write!(f, "{adj}"),
        }
    }
}

// obsolete and wrong (missing 'the' prefix)
/*impl From<Token> for Prefix {
    fn from(token: Token) -> Self {
        match token.name {
            TokenType::Not => Prefix::Not,
            TokenType::Minus => Prefix::Negation,
            TokenType::Identifier => Prefix::Adjective(token.lexeme),
            _ => Prefix::None,
        }
    }
}*/
