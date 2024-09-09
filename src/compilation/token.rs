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
