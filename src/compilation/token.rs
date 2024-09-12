use core::slice::Iter;
use std::collections::HashMap;
use std::iter::Peekable;
use std::fmt;
use std::rc::Rc;
use serde::{Deserialize, Serialize};

use crate::compilation::errors::CompilerError;

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
            ("or".into(), TokenType::Or),
            ("the".into(), TokenType::The),
            ("to".into(), TokenType::To),
            ("true".into(), TokenType::True),
            ("verb".into(), TokenType::Verb),
            ("when".into(), TokenType::When),
        ]).into()
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
    Bang,
  
    // One or two character tokens.
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
  
    // Literals.
    Identifier,
    Number,
    Text,
    Type,
  
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
