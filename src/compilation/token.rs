use core::slice::Iter;
use std::collections::HashMap;
use std::iter::Peekable;
use std::fmt;
use std::rc::Rc;
use serde::{Deserialize, Serialize};

use crate::compilation::datatype::Datatype;
use crate::compilation::errors::CompilerError;
use crate::compilation::precedent::Precedent;

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
            ("adjective".into(), TokenType::Adjective),
            ("and".into(), TokenType::And),
            ("as".into(), TokenType::As),
            ("false".into(), TokenType::False),
            ("for".into(), TokenType::For),
            ("hence".into(), TokenType::Hence),
            ("is".into(), TokenType::Is),
            ("it".into(), TokenType::It),
            ("noun".into(), TokenType::Noun),
            ("not".into(), TokenType::Not),
            ("notion".into(), TokenType::Type(Datatype::Notion)),
            ("number".into(), TokenType::Type(Datatype::Number)),
            ("or".into(), TokenType::Or),
            ("so".into(), TokenType::So),
            ("text".into(), TokenType::Type(Datatype::Text)),
            ("the".into(), TokenType::The),
            ("to".into(), TokenType::To),
            ("true".into(), TokenType::True),
            ("verb".into(), TokenType::Verb),
            ("when".into(), TokenType::When),
        ]).into()
    }
}

pub enum TokenCategory {
    Atom(Token),
    Op(Token),
    EOF,
}

impl From<Token> for TokenCategory {
    fn from(value: Token) -> Self {
        match &value.name {
            TokenType::None => TokenCategory::Atom(value),
            TokenType::Identifier => TokenCategory::Atom(value),
            TokenType::Number => TokenCategory::Atom(value),
            TokenType::False => TokenCategory::Atom(value),
            TokenType::True => TokenCategory::Atom(value),
            TokenType::Text => TokenCategory::Atom(value),
            TokenType::Type(_) => TokenCategory::Atom(value),
            TokenType::It => TokenCategory::Atom(value),

            TokenType::LeftParen => TokenCategory::Op(value),
            TokenType::RightParen => TokenCategory::Op(value),
            TokenType::LeftBrace => TokenCategory::Op(value),
            TokenType::RightBrace => TokenCategory::Op(value),
            TokenType::Comma => TokenCategory::Op(value),
            TokenType::Dot => TokenCategory::Op(value),
            TokenType::Minus =>TokenCategory::Op(value),
            TokenType::Plus => TokenCategory::Op(value),
            TokenType::Slash =>TokenCategory::Op(value),
            TokenType::Star => TokenCategory::Op(value),
            TokenType::Equal => TokenCategory::Op(value),
            TokenType::Tilde => TokenCategory::Op(value),
            TokenType::Greater => TokenCategory::Op(value),
            TokenType::GreaterEqual => TokenCategory::Op(value),
            TokenType::Less => TokenCategory::Op(value),
            TokenType::LessEqual => TokenCategory::Op(value),

            TokenType::Adjective => TokenCategory::Op(value),
            TokenType::And => TokenCategory::Op(value),
            TokenType::As => TokenCategory::Op(value),
            TokenType::For => TokenCategory::Op(value),
            TokenType::Hence => TokenCategory::Op(value),
            TokenType::Is => TokenCategory::Op(value),
            TokenType::Noun => TokenCategory::Op(value),
            TokenType::Not => TokenCategory::Op(value),
            TokenType::Or => TokenCategory::Op(value),
            TokenType::So => TokenCategory::Op(value),
            TokenType::The => TokenCategory::Op(value),
            TokenType::To => TokenCategory::Op(value),
            TokenType::Verb => TokenCategory::Op(value),
            TokenType::When => TokenCategory::Op(value),

            TokenType::EOF => TokenCategory::EOF,
        }
    }
}

pub trait TokenCollection {
    fn add(&mut self, token: TokenType, text: Option<&str>, line: u32);
}

impl TokenCollection for Vec<Token> {
    fn add(&mut self, token: TokenType, text: Option<&str>, line: u32) {
        if token == TokenType::None { return; }

        let literal = if let Some(txt) = text {
            if token == TokenType::Text {
                let trimmed = txt.trim_matches('\"');
                Some(trimmed.to_string())
            }
            else if token == TokenType::Number {
                let trimmed = txt.trim_start_matches('[').trim_end_matches(']');
                let number = trimmed.parse::<f32>().unwrap_or_default();
                let number = if number.fract() > f32::EPSILON {
                    format!("{}", number)
                } else {
                    format!("{:.1}", number)
                };
                Some(number)
            }
            else {
                Some(txt.to_string())
            }
        }
        else { None };

        self.push(Token {
            name: token, 
            lexeme: literal.unwrap_or_default().into(),
            line,
        });
    }
}

#[derive(Default, PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum TokenType {
    #[default] None,

    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,
    Equal,
    Tilde,
    //Bang, Used for comment, not tokenized
  
    // One or two character tokens.
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
  
    // Literals.
    Identifier,
    Number,
    Text,
    Type(Datatype),
  
    // Keywords.
    Adjective,
    And,
    As,
    False,
    For,
    Hence,
    Is,
    It,
    Noun,
    Not,
    Or,
    So,
    The,
    To,
    True,
    Verb,
    When,
  
    EOF
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TokenType {
    pub fn precedent(&self) -> Precedent {
        match self {
            TokenType::Identifier => Precedent::Infix(1, 2),
            TokenType::Comma => Precedent::Infix(1, 2),

            TokenType::When => Precedent::Postfix(3),

            TokenType::As => Precedent::Infix(5, 4),

            TokenType::Or => Precedent::Infix(6, 7),
            TokenType::And => Precedent::Infix(8, 9),
            TokenType::Equal | TokenType::Tilde => Precedent::Infix(10, 11),
            TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual => Precedent::Infix(12, 13),

            TokenType::Minus => Precedent::Infix(14, 15),
            TokenType::Plus => Precedent::Infix(14, 15),
            TokenType::Slash | TokenType::Star => Precedent::Infix(16, 17),

            TokenType::Not => Precedent::Prefix(18),
            TokenType::The => Precedent::Prefix(19),

            // Separated in case refactor make these ones have precedent
            TokenType::LeftParen => Precedent::None,
            TokenType::RightParen => Precedent::None,
            TokenType::LeftBrace => Precedent::None,
            TokenType::RightBrace => Precedent::None,

            TokenType::Adjective => Precedent::None,
            TokenType::Dot => Precedent::None,
            TokenType::For => Precedent::None,
            TokenType::Hence => Precedent::None,
            TokenType::Is => Precedent::None,
            TokenType::Noun => Precedent::None,
            TokenType::So => Precedent::None,
            TokenType::To => Precedent::None,
            TokenType::Verb => Precedent::None,

            _ => Precedent::None,
        }
    }
}

pub trait TokenBuffer {
    fn is_at_end(&mut self) -> bool;
    fn peek_next(&mut self, target: TokenType) -> bool;
    fn match_next(&mut self, target: &[TokenType]) -> bool;
    fn consume(&mut self, target: TokenType) -> Result<&Token, CompilerError>;
    fn get_current(&mut self) -> Option<&Token>;
    fn advance(&mut self);
}

impl TokenBuffer for Peekable<Iter<'_, Token>> {
    fn is_at_end(&mut self) -> bool {
        let current = self.peek();
        current.is_none() || current
            .and_then(|&token| Some(token.name == TokenType::EOF))
            .unwrap_or_default()
    }

    fn peek_next(&mut self, target: TokenType) -> bool {
        if self.is_at_end() {
            return false; 
        }
        
        let current = self.peek();
        if let Some(tok) = current {
            tok.name == target
        } else {
            false
        }
    }
    
    fn match_next(&mut self, target: &[TokenType]) -> bool {
        for token in target {
            if self.next_if(|&next| next.name == *token).is_some() {
                return true;
            }
        }

        false
    }

    fn consume(&mut self, target: TokenType) -> Result<&Token, CompilerError> {
        match self.next_if(|next| next.name == target) {
            Some(token) => Ok(token),
            None => {
                if let Some(current) = self.peek() {
                    let msg = format!("[line {}] Error at '{}': Expect {}.", current.line, current.lexeme, target);
                    Err(CompilerError::LexicalError(msg.into()))
                } else {
                    Err(CompilerError::LexicalError("Consuming token at end of file".into()))
                }
            },
        }
    }

    fn get_current(&mut self) -> Option<&Token> {
        self.peek().map(|&token| token)
    }

    fn advance(&mut self) {
        self.next();
    }
}
