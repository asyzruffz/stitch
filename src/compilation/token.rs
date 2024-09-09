use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Token {
    pub name: TokenType,
    pub lexeme: Rc<str>,
    pub line: u32,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.name, self.lexeme)
    }
}

impl Token {
    pub fn keywords() -> HashMap<Rc<str>, TokenType> {
        HashMap::from([
            ("and".into(), TokenType::And),
            ("class".into(), TokenType::Class),
            ("else".into(), TokenType::Else),
            ("false".into(), TokenType::False),
            ("for".into(), TokenType::For),
            ("fun".into(), TokenType::Fun),
            ("if".into(), TokenType::If),
            ("nil".into(), TokenType::Nil),
            ("or".into(), TokenType::Or),
            ("print".into(), TokenType::Print),
            ("return".into(), TokenType::Return),
            ("super".into(), TokenType::Super),
            ("this".into(), TokenType::This),
            ("true".into(), TokenType::True),
            ("var".into(), TokenType::Var),
            ("while".into(), TokenType::While),
        ]).into()
    }
}

#[derive(Default, PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum TokenType {
    #[default] None,

    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
  
    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
  
    // Literals.
    Identifier, String, Number,
  
    // Keywords.
    And, Class, Else, False, For, Fun, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
  
    EOF
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
