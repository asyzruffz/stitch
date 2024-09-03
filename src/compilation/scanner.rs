use std::collections::HashMap;
use std::io::{self, Write};
use std::str::FromStr;

use super::token::{Token, TokenType};

pub trait ScannerState {}

pub struct Scanner<State: ScannerState = Initial> {
    state: State,
}

#[derive(Debug, Default)]
pub struct Initial;
#[derive(Debug, Default)]
pub struct Ready {
    pub source : String,
}
#[derive(Debug, Default)]
pub struct Done {
    pub tokens : Vec<Token>,
    pub error_count : u32,
}

impl ScannerState for Initial {}
impl ScannerState for Ready {}
impl ScannerState for Done {}

impl Scanner<Initial> {
    pub fn new(source: String) -> Scanner<Ready> {
        Scanner::<Ready> {
            state: Ready { source },
        }
    }
}

impl Scanner<Ready> {
    pub fn tokenize(&mut self) -> Scanner<Done> {
        let source = &self.state.source;
        let mut _start = 0usize;
        let mut current = 0usize;
        let mut line = 1u32;
        
        let mut tokens : Vec<Token> = Vec::new();
        let mut error_count = 0u32;
        
        let keywords = Token::keywords();

        while !Self::is_at_end(source, current) {
            // We are at the beginning of the next lexeme.
            _start = current;

            let token = Self::scan_token(source, _start, &mut current, &mut line, &keywords, &mut error_count);
            let text = Self::extract_text(source, _start, current);
            
            tokens = Self::add_token(tokens, token, text, line);
        }

        tokens = Self::add_token(tokens, TokenType::EOF, None, line);

        Scanner::<Done> {
            state: Done {
                tokens,
                error_count,
            },
        }
    }

    fn is_at_end(source: &String, current : usize) -> bool {
        current >= source.len()
    }

    fn scan_token(source: &String, start : usize, current: &mut usize, line: &mut u32, keywords: &HashMap<String, TokenType>, error_count: &mut u32) -> TokenType {
        if Self::is_at_end(source, *current) {
            return TokenType::EOF; 
        }

        let symbol = Self::extract_symbol_at(source, *current);
        *current += 1;

        match symbol {
            Some('(') => TokenType::LeftParen,
            Some(')') => TokenType::RightParen,
            Some('{') => TokenType::LeftBrace,
            Some('}') => TokenType::RightBrace,
            Some(',') => TokenType::Comma,
            Some('.') => TokenType::Dot,
            Some('-') => TokenType::Minus,
            Some('+') => TokenType::Plus,
            Some(';') => TokenType::Semicolon,
            Some('*') => TokenType::Star,
            Some('!') => if Self::match_next(source, current, '=') { TokenType::BangEqual } else { TokenType::Bang },
            Some('=') => if Self::match_next(source, current, '=') { TokenType::EqualEqual } else { TokenType::Equal },
            Some('<') => if Self::match_next(source, current, '=') { TokenType::LessEqual } else { TokenType::Less },
            Some('>') => if Self::match_next(source, current, '=') { TokenType::GreaterEqual } else { TokenType::Greater },
            Some('/') => if Self::match_next(source, current, '/') { Self::skip_comment(source, current) } else { TokenType::Slash },
            // Skip whitespaces
            Some(' ') => TokenType::None,
            Some('\r') => TokenType::None,
            Some('\t') => TokenType::None,
            Some('\n') => { *line += 1; TokenType::None },
            Some('\"') => Self::handle_string(source, current, line, error_count),
            Some(c) => if Self::is_digit(c) {
                Self::handle_number(source, current)
            } else if Self::is_alpha(c) {
                Self::handle_identifier(source, start, current, keywords)
            } else {
                writeln!(io::stderr(), "[line {}] Error: Unexpected character: {}", line, c).unwrap();
                *error_count += 1;
                TokenType::None
            },
            none => {
                writeln!(io::stderr(), "[line {}] Error: Unexpected character: {:?}", line, none).unwrap();
                *error_count += 1;
                TokenType::None
            }
        }
    }

    fn add_token(mut tokens : Vec<Token>, token: TokenType, text: Option<String>, line: u32) -> Vec<Token> {
        if token == TokenType::None { return tokens; }

        let literal = text.clone().and_then(|t| {
            if token == TokenType::String { Some(t[1..(t.len()-1)].to_string()) }
            else if token == TokenType::Number {
                let number = t.parse::<f32>().unwrap_or_default();
                let number = if number.fract() > f32::EPSILON {
                    format!("{}", number)
                } else {
                    format!("{:.1}", number)
                };
                Some(number)
            }
            else { None }
        });

        tokens.push(Token {
            name: token, 
            lexeme: text.unwrap_or_default(),
            literal,
            line,
        });

        tokens
    }

    fn extract_text(source: &String, from: usize, to: usize) -> Option<String> {
        source.get(from..to)
            .and_then(|t| Some(String::from_str(t).unwrap()))
    }

    fn extract_symbol_at(source: &String, pos: usize) -> Option<char> {
        source.get(pos..pos+1)
            .and_then(|c| c.chars().next())
    }

    fn match_next(source: &String, current : &mut usize, symbol: char) -> bool {
        if !Self::peek_next(source, *current, symbol) {
            return false;
        }

        *current += 1;
        true
    }

    fn peek_next(source: &String, current : usize, symbol: char) -> bool {
        if Self::is_at_end(source, current) {
            return false; 
        }
        
        let next = Self::extract_symbol_at(source, current);
        next == Some(symbol)
    }
    
    fn skip_comment(source: &String, current : &mut usize) -> TokenType {
        // A comment goes until the end of the line.
        while !Self::peek_next(source, *current, '\n') && !Self::is_at_end(source, *current) { 
            *current += 1;
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
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn handle_string(source: &String, current : &mut usize, line : &mut u32, error_count : &mut u32) -> TokenType {
        while !Self::peek_next(source, *current, '\"') && !Self::is_at_end(source, *current) {
            if Self::peek_next(source, *current, '\n') { *line += 1; }
            *current += 1;
        }

        if Self::is_at_end(source, *current) {
            writeln!(io::stderr(), "[line {}] Error: Unterminated string.", line).unwrap();
            *error_count += 1;
            return TokenType::None; 
        }

        *current += 1;
        TokenType::String
    }

    fn handle_number(source: &String, current : &mut usize) -> TokenType {
        while Self::is_digit(Self::extract_symbol_at(source, *current).unwrap_or_default()) {
            *current += 1;
        }

        if Self::extract_symbol_at(source, *current) == Some('.') && Self::is_digit(Self::extract_symbol_at(source, *current + 1).unwrap_or_default()) {
            *current += 1;
            while Self::is_digit(Self::extract_symbol_at(source, *current).unwrap_or_default()) {
                *current += 1;
            }
        }

        TokenType::Number
    }

    fn handle_identifier(source: &String, start : usize, current : &mut usize, keywords: &HashMap<String, TokenType>) -> TokenType {
        while Self::is_alphanumeric(Self::extract_symbol_at(source, *current).unwrap_or_default()) {
            *current += 1;
        }

        let text = Self::extract_text(source, start, *current);
        keywords.get(&text.unwrap_or_default())
            .unwrap_or(&TokenType::Identifier)
            .clone()
    }
}

impl Scanner<Done> {
    pub fn tokens_ref(&self) -> &Vec<Token> {
        &self.state.tokens
    }
    
    pub fn tokens(self) -> Vec<Token> {
        self.state.tokens
    }

    pub fn is_err(&self) -> bool {
        self.state.error_count > 0
    }

    pub fn error_count(&self) -> u32 {
        self.state.error_count
    }
}
