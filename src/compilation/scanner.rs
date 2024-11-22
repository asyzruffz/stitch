use std::collections::HashMap;
use std::rc::Rc;

use crate::compilation::source::SourceBuffer;
use crate::compilation::token::{Token, TokenType, TokenCollection};
use crate::compilation::intermediate::Intermediate;

pub trait ScannerState {}

pub struct Scanner<State: ScannerState = Initial> {
    state: State,
}

#[derive(Debug, Default)]
pub struct Initial;
#[derive(Debug, Default)]
pub struct Ready {
    pub source : Rc<str>,
    pub hash : Rc<[u8]>,
}
#[derive(Debug, Default)]
pub struct Done {
    pub intermediate : Intermediate,
    pub error_count : u32,
}

impl ScannerState for Initial {}
impl ScannerState for Ready {}
impl ScannerState for Done {}

impl Scanner<Initial> {
    pub fn new(source: &str, hash: Rc<[u8]>) -> Scanner<Ready> {
        Scanner::<Ready> {
            state: Ready {
                source: source.into(),
                hash: hash.into(),
            },
        }
    }

    pub fn from(intermediate: Intermediate) -> Scanner<Done> {
        Scanner::<Done> {
            state: Done {
                intermediate,
                error_count: 0,
            },
        }
    }
}

impl Scanner<Ready> {
    pub fn tokenize(&mut self) -> Scanner<Done> {
        let mut buffer = SourceBuffer::from(self.state.source.chars());
        let mut line = 1u32;
        
        let mut tokens : Vec<Token> = Vec::new();
        let mut error_count = 0u32;
        
        let keywords = Token::keywords();

        while !buffer.is_at_end() {
            // We are at the beginning of the next lexeme.
            buffer.start();

            let token = scan_token(&mut buffer, &mut line, &keywords, &mut error_count);
            let text = buffer.extract();
            
            tokens.add(token, Some(text.as_str()), line);
        }

        tokens.add(TokenType::EOF, None, line);

        Scanner::<Done> {
            state: Done {
                intermediate: Intermediate::new(tokens.as_slice(), self.state.hash.clone()),
                error_count,
            },
        }
    }
}

impl Scanner<Done> {
    pub fn intermediate(&self) -> &Intermediate {
        &self.state.intermediate
    }
    
    pub fn is_err(&self) -> bool {
        self.state.error_count > 0
    }

    pub fn error_count(&self) -> u32 {
        self.state.error_count
    }
}

fn scan_token(source: &mut SourceBuffer, line: &mut u32, keywords: &HashMap<Rc<str>, TokenType>, error_count: &mut u32) -> TokenType {
    if source.is_at_end() {
        return TokenType::EOF; 
    }

    let symbol = source.next();
    match symbol {
        Some('(') => TokenType::LeftParen,
        Some(')') => TokenType::RightParen,
        Some('{') => TokenType::LeftBrace,
        Some('}') => TokenType::RightBrace,
        Some(',') => TokenType::Comma,
        Some('.') => TokenType::Dot,
        Some('-') => TokenType::Minus,
        Some('+') => TokenType::Plus,
        Some('*') => TokenType::Star,
        Some('/') => TokenType::Slash,
        Some('=') => TokenType::Equal,
        Some('~') => TokenType::Tilde,
        Some('<') => if source.match_next('=') { TokenType::LessEqual } else { TokenType::Less },
        Some('>') => if source.match_next('=') { TokenType::GreaterEqual } else { TokenType::Greater },
        Some('!') => skip_comment(source),
        // Skip whitespaces
        Some(' ') => TokenType::None,
        Some('\r') => TokenType::None,
        Some('\t') => TokenType::None,
        Some('\n') => { *line += 1; TokenType::None },
        Some('\"') => handle_text(source, line, error_count),
        Some('[') => handle_number(source, line, error_count),
        Some(c) => if is_alpha(c) {
            handle_identifier(source, keywords)
        } else {
            eprintln!("[line {}] Error: Unexpected character: {}", line, c);
            if is_digit(c) { eprintln!("    Wrap any number literal with [ ]"); }
            *error_count += 1;
            TokenType::None
        },
        none => {
            eprintln!("[line {}] Error: Unexpected character: {:?}", line, none);
            *error_count += 1;
            TokenType::None
        }
    }
}

fn handle_text(source: &mut SourceBuffer, line : &mut u32, error_count : &mut u32) -> TokenType {
    while !source.peek_next('\"') && !source.is_at_end() {
        if source.peek_next('\n') { *line += 1; }
        source.next();
    }

    if source.is_at_end() {
        eprintln!("[line {}] Error: Unterminated text.", line);
        *error_count += 1;
        return TokenType::None; 
    }

    source.next();
    TokenType::Text
}

fn handle_number(source: &mut SourceBuffer, line : &mut u32, error_count : &mut u32) -> TokenType {
    while let Some(_) = source.next_if(|&next| is_digit(next)) {}

    if source.match_next('.') && source.next_if(|&next| is_digit(next)).is_some() {
        while let Some(_) = source.next_if(|&next| is_digit(next)) {}
    }

    if !source.match_next(']') || source.is_at_end() {
        eprintln!("[line {}] Error: Unterminated number.", line);
        *error_count += 1;
        return TokenType::None; 
    }

    TokenType::Number
}

fn handle_identifier(source: &mut SourceBuffer, keywords: &HashMap<Rc<str>, TokenType>) -> TokenType {
    while let Some(_) = source.next_if(|&next| is_alphanumeric(next)) {}

    let text = source.extract();
    keywords.get(text.as_str())
        .unwrap_or(&TokenType::Identifier)
        .clone()
}

fn skip_comment(source: &mut SourceBuffer) -> TokenType {
    // A comment goes until the end of the line.
    while !source.match_next('\n') && !source.is_at_end() { 
        source.next();
    }
    TokenType::None
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn is_alpha(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}
