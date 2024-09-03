use std::collections::HashMap;
use std::fmt;

#[derive(Default, Clone, Debug)]
pub struct Token {
    pub name: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: u32,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", 
            self.name,
            self.lexeme, 
            self.literal.clone().unwrap_or("null".to_string()))
    }
}

impl Token {
    pub fn keywords() -> HashMap<String, TokenType> {
        HashMap::from([
            ("and".to_string(), TokenType::And),
            ("class".to_string(), TokenType::Class),
            ("else".to_string(), TokenType::Else),
            ("false".to_string(), TokenType::False),
            ("for".to_string(), TokenType::For),
            ("fun".to_string(), TokenType::Fun),
            ("if".to_string(), TokenType::If),
            ("nil".to_string(), TokenType::Nil),
            ("or".to_string(), TokenType::Or),
            ("print".to_string(), TokenType::Print),
            ("return".to_string(), TokenType::Return),
            ("super".to_string(), TokenType::Super),
            ("this".to_string(), TokenType::This),
            ("true".to_string(), TokenType::True),
            ("var".to_string(), TokenType::Var),
            ("while".to_string(), TokenType::While),
        ]).into()
    }
}

#[derive(Default, PartialEq, Eq, Clone, Debug)]
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
