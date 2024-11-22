use std::rc::Rc;

use crate::compilation::token::{Token, TokenCategory, TokenType, TokenBuffer};
use crate::compilation::datatype::Datatype;
use crate::compilation::phrase::Phrase;
use crate::compilation::primitive::Primitive;
use crate::compilation::prefix::Prefix;
use crate::compilation::statement::{Statement, Statements};
use crate::compilation::errors::CompilerError;

use super::precedent::Precedent;

pub trait ParserState {}

pub struct Parser<State: ParserState = Initial> {
    state: State,
}

#[derive(Debug, Default)]
pub struct Initial;
#[derive(Debug, Default)]
pub struct Ready {
    pub tokens : Rc<[Token]>,
}
#[derive(Debug, Default)]
pub struct Done {
    pub statements : Rc<[Statement]>,
}

impl ParserState for Initial {}
impl ParserState for Ready {}
impl ParserState for Done {}

impl Parser<Initial> {
    pub fn new(tokens : Rc<[Token]>) -> Parser<Ready> {
        Parser::<Ready> {
            state: Ready { tokens },
        }
    }
}

impl Parser<Ready> {
    pub fn parse(&mut self) -> Result<Parser<Done>, CompilerError> {
        let mut buffer = self.state.tokens.iter().peekable();

        let mut statements = Vec::<Statement>::new();
        let mut errors = CompilerError::None;
        
        while !buffer.is_at_end() {
            match handle_prose(&mut buffer) {
                Ok(statement) => statements.push(statement),
                Err(error) => errors = errors.add(error),
            }
        }

        if errors == CompilerError::None {
            Ok(Parser::<Done> {
                state: Done {
                    statements: statements.into(),
                },
            })
        } else {
            Err(errors)
        }
    }
}

impl Parser<Done> {
    pub fn statements(&self) -> Rc<[Statement]> {
        self.state.statements.clone()
    }
}

fn handle_prose<'a, Buffer>(tokens : &mut Buffer) -> Result<Statement, CompilerError>
    where Buffer: TokenBuffer + Iterator<Item = &'a Token> {
    if tokens.peek_next(TokenType::Noun)
        || tokens.peek_next(TokenType::Verb)
        || tokens.peek_next(TokenType::Adjective)
        || tokens.peek_next(TokenType::So) {
        handle_definition(tokens)
    } else {
        handle_sentence(tokens)
    }
}

fn handle_definition<'a, Buffer>(tokens : &mut Buffer) -> Result<Statement, CompilerError>
    where Buffer: TokenBuffer + Iterator<Item = &'a Token> {
    if tokens.match_next(&[TokenType::Noun]) {
        return handle_noun_definition(tokens);
    } else if tokens.match_next(&[TokenType::Verb]) {
        return handle_verb_definition(tokens);
    } else if tokens.match_next(&[TokenType::Adjective]) {
        return handle_adjective_definition(tokens);
    } else if tokens.match_next(&[TokenType::So]) {
        return handle_so_definition(tokens);
    }

    let token = match tokens.get_current() {
        Some(token) => token.to_owned(),
        None => {
            return Err(CompilerError::LexicalError("Unexpected EOF".into()));
        }
    };

    tokens.advance();
    let msg = format!("At '{}' [line {}], invalid definition", token.lexeme, token.line);
    Err(CompilerError::LexicalError(msg.into()))
}

fn handle_noun_definition<'a, Buffer>(tokens : &mut Buffer) -> Result<Statement, CompilerError>
    where Buffer: TokenBuffer + Iterator<Item = &'a Token> {
    let name_token = match tokens.consume(TokenType::Identifier) {
        Ok(token) => token.to_owned(),
        Err(error) => return Err(error),
    };

    let super_type = if let Ok(_) = tokens.consume(TokenType::Is) {
        let datatype = match tokens.next() {
            Some(Token { name: TokenType::Type(datatype), .. }) => datatype.to_owned(),
            Some(Token { name: TokenType::Identifier, lexeme, .. }) => Datatype::Custom(lexeme.to_owned().into()),
            token => {
                let msg = format!("Invalid datatype {token:?}");
                return Err(CompilerError::LexicalError(msg.into()));
            },
        };

        Some(datatype)
    } else {
        None
    };

    if let Err(error) = tokens.consume(TokenType::LeftBrace) {
        return Err(error);
    }

    let mut definitions = Vec::new();
    while !tokens.peek_next(TokenType::RightBrace) && !tokens.is_at_end() {
        let definition = handle_definition(tokens)?;
        definitions.push(definition);
    }

    if let Err(error) = tokens.consume(TokenType::RightBrace) {
        return Err(error);
    }

    Ok(Statement::Noun {
        name: name_token.lexeme,
        super_type,
        body: Statements(definitions.into()),
    })
}

fn handle_verb_definition<'a, Buffer>(tokens : &mut Buffer) -> Result<Statement, CompilerError>
    where Buffer: TokenBuffer + Iterator<Item = &'a Token> {
    let name_token = match tokens.consume(TokenType::Identifier) {
        Ok(token) => token.to_owned(),
        Err(error) => return Err(error),
    };

    let hence_type = if let Ok(_) = tokens.consume(TokenType::Is) {
        let datatype = match tokens.next() {
            Some(Token { name: TokenType::Type(datatype), .. }) => datatype.to_owned(),
            Some(Token { name: TokenType::Identifier, lexeme, .. }) => Datatype::Custom(lexeme.to_owned().into()),
            token => {
                let msg = format!("Invalid datatype {token:?}");
                return Err(CompilerError::LexicalError(msg.into()));
            },
        };

        Some(datatype)
    } else {
        None
    };

    let subject_type = if let Ok(_) = tokens.consume(TokenType::For) {
        let datatype = match tokens.next() {
            Some(Token { name: TokenType::Type(datatype), .. }) => datatype.to_owned(),
            Some(Token { name: TokenType::Identifier, lexeme, .. }) => Datatype::Custom(lexeme.to_owned().into()),
            token => {
                let msg = format!("Invalid datatype {token:?}");
                return Err(CompilerError::LexicalError(msg.into()));
            },
        };

        Some(datatype)
    } else {
        None
    };

    let mut parameters = Vec::new();
    if let Ok(_) = tokens.consume(TokenType::When) {
        parameters = handle_parameters(tokens)?;
    }

    if let Err(error) = tokens.consume(TokenType::LeftBrace) {
        return Err(error);
    }

    let mut sentences = Vec::new();
    while !tokens.peek_next(TokenType::RightBrace) && !tokens.is_at_end() {
        let sentence = handle_sentence(tokens)?;
        sentences.push(sentence);
    }

    if let Err(error) = tokens.consume(TokenType::RightBrace) {
        return Err(error);
    }

    Ok(Statement::Verb {
        name: name_token.lexeme,
        hence_type,
        subject_type,
        object_types: parameters.into(),
        body: Statements(sentences.into()),
    })
}

fn handle_parameters<'a, Buffer>(tokens : &mut Buffer) -> Result<Vec<Statement>, CompilerError>
    where Buffer: TokenBuffer + Iterator<Item = &'a Token> {
    let mut declarations = Vec::new();

    if let Err(error) = tokens.consume(TokenType::So) {
        return Err(error);
    }

    declarations.push(handle_so_declaration(tokens)?);

    while tokens.match_next(&[TokenType::Comma]) {
        if tokens.match_next(&[TokenType::And]) {
            if let Err(error) = tokens.consume(TokenType::So) {
                return Err(error);
            }
        
            declarations.push(handle_so_declaration(tokens)?);
            break;
        } else {
            if let Err(error) = tokens.consume(TokenType::So) {
                return Err(error);
            }
        
            declarations.push(handle_so_declaration(tokens)?);
        }
    };

    Ok(declarations)
}

fn handle_adjective_definition<'a, Buffer>(tokens : &mut Buffer) -> Result<Statement, CompilerError>
    where Buffer: TokenBuffer + Iterator<Item = &'a Token> {
    let name_token = match tokens.consume(TokenType::Identifier) {
        Ok(token) => token.to_owned(),
        Err(error) => return Err(error),
    };

    let subject_type = if let Ok(_) = tokens.consume(TokenType::For) {
        let datatype = match tokens.next() {
            Some(Token { name: TokenType::Type(datatype), .. }) => datatype.to_owned(),
            Some(Token { name: TokenType::Identifier, lexeme, .. }) => Datatype::Custom(lexeme.to_owned().into()),
            token => {
                let msg = format!("Invalid datatype {token:?}");
                return Err(CompilerError::LexicalError(msg.into()));
            },
        };

        datatype
    } else {
        return Err(CompilerError::LexicalError("Adjective missing subject datatype".into()));
    };

    if let Err(error) = tokens.consume(TokenType::LeftBrace) {
        return Err(error);
    }

    let mut sentences = Vec::new();
    while !tokens.peek_next(TokenType::RightBrace) && !tokens.is_at_end() {
        let sentence = handle_sentence(tokens)?;
        sentences.push(sentence);
    }

    if let Err(error) = tokens.consume(TokenType::RightBrace) {
        return Err(error);
    }

    Ok(Statement::Adjective {
        name: name_token.lexeme,
        subject_type,
        body: Statements(sentences.into()),
    })
}

fn handle_so_definition<'a, Buffer>(tokens : &mut Buffer) -> Result<Statement, CompilerError>
    where Buffer: TokenBuffer + Iterator<Item = &'a Token> {
    let definition = handle_so_declaration(tokens)?;

    if let Err(error) = tokens.consume(TokenType::Dot) {
        return Err(error);
    }

    Ok(definition)
}

fn handle_so_declaration<'a, Buffer>(tokens : &mut Buffer) -> Result<Statement, CompilerError>
    where Buffer: TokenBuffer + Iterator<Item = &'a Token> {
    let name_token = match tokens.consume(TokenType::Identifier) {
        Ok(token) => token.to_owned(),
        Err(error) => return Err(error),
    };

    if let Err(error) = tokens.consume(TokenType::Is) {
        return Err(error);
    }

    let datatype = match tokens.next() {
        Some(Token { name: TokenType::Type(datatype), .. }) => datatype.to_owned(),
        Some(Token { name: TokenType::Identifier, lexeme, .. }) => Datatype::Custom(lexeme.to_owned().into()),
        token => {
            let msg = format!("Invalid datatype {token:?}");
            return Err(CompilerError::LexicalError(msg.into()));
        },
    };

    let token = match tokens.get_current() {
        Some(token) => token.to_owned(),
        None => {
            return Err(CompilerError::LexicalError("Unexpected EOF".into()));
        }
    };

    let initializer = if tokens.match_next(&[TokenType::As]) {
        if let Precedent::Infix(_, r_bp) = token.name.precedent() {
            Some(handle_phrase(tokens, r_bp)?)
        } else { 
            let msg = format!("[line {}] Error at '{}': {} has a wrong precedent type.", token.line, token.lexeme, token.name);
            return Err(CompilerError::LexicalError(msg.into()))
        }
    } else { None };

    Ok(Statement::So {
        name: name_token.lexeme,
        datatype,
        initializer
    })
}

fn handle_sentence<'a, Buffer>(tokens : &mut Buffer) -> Result<Statement, CompilerError>
    where Buffer: TokenBuffer + Iterator<Item = &'a Token> {
    let hence = tokens.match_next(&[TokenType::Hence]);
    let phrase = handle_phrase(tokens, 0)?;

    if let Err(error) = tokens.consume(TokenType::Dot) {
        return Err(error);
    }

    if hence {
        Ok(Statement::Hence(phrase))
    } else {
        Ok(Statement::Phrase(phrase))
    }
}

fn handle_phrase<'a, Buffer>(tokens : &mut Buffer, precedent: u8) -> Result<Phrase, CompilerError>
    where Buffer: TokenBuffer + Iterator<Item = &'a Token> {
    let category = tokens.next().map(|t| TokenCategory::from(t.to_owned()));

    let mut phrase = match category {
        Some(TokenCategory::Atom(token)) => handle_atom(token)?,
        Some(TokenCategory::Op(prefix)) => handle_prefix(tokens, prefix)?,
        Some(TokenCategory::EOF) => {
            return Err(CompilerError::LexicalError("Unexpected EOF".into()));
        },
        None => {
            return Err(CompilerError::LexicalError("Unexpected EOF".into()));
        }
    };

    loop {
        // Check if there is an operator after the phrase, break the loop otherwise
        let op = match tokens.get_current().map(|t| TokenCategory::from(t.to_owned())) {
            Some(TokenCategory::Op(token)) => token,
            Some(TokenCategory::Atom(token @ Token { name: TokenType::Identifier, .. })) => token,
            Some(TokenCategory::Atom(token)) => {
                let msg = format!("[line {}] Error at '{}': {} is invalid operator.", token.line, token.lexeme, token.name);
                return Err(CompilerError::LexicalError(msg.into()));
            },
            Some(TokenCategory::EOF) => break,
            None => break,
        };

        match op.name.precedent() {
            Precedent::Postfix(l_bp) => {
                if l_bp < precedent { break; }
    
                phrase = handle_postfix(tokens, phrase)?;

                continue;
            },
            Precedent::Infix(l_bp, r_bp) => {
                if l_bp < precedent { break; }
                tokens.next();

                let object = handle_collective(tokens, r_bp)?;
    
                phrase = Phrase::Action {
                    subject: Some(Box::new(phrase)),
                    verb: op.name.into(),
                    object: Some(Box::new(object)),
                };

                continue;
            },
            _ => { break; },
        };
    }

    Ok(phrase)
}

fn handle_collective<'a, Buffer>(tokens : &mut Buffer, precedent: u8) -> Result<Phrase, CompilerError> 
    where Buffer: TokenBuffer + Iterator<Item = &'a Token> {
    let mut phrases = Vec::<Phrase>::new();
    phrases.push(handle_phrase(tokens, precedent)?);

    match tokens.get_current().cloned() {
        None => {
            phrases.first()
                .map(|phr| phr.to_owned())
                .ok_or(CompilerError::LexicalError("Empty collective".into()))
        }
        Some(current) => {
            let token = current;

            while tokens.match_next(&[TokenType::Comma]) {

                if tokens.match_next(&[TokenType::And, TokenType::Or]) {
                    if let Precedent::Infix(_, r_bp) = token.name.precedent() {
                        phrases.push(handle_phrase(tokens, r_bp)?);
                    } else { 
                        let msg = format!("[line {}] Error at '{}': {} has a wrong precedent type.", token.line, token.lexeme, token.name);
                        return Err(CompilerError::LexicalError(msg.into()))
                    }

                    break;
                } else {
                    if let Precedent::Infix(_, r_bp) = token.name.precedent() {
                        phrases.push(handle_phrase(tokens, r_bp)?);
                    } else { 
                        let msg = format!("[line {}] Error at '{}': {} has a wrong precedent type.", token.line, token.lexeme, token.name);
                        return Err(CompilerError::LexicalError(msg.into()))
                    }
                }
            };

            if phrases.len() == 1 {
                phrases.first()
                    .map(|phr| phr.to_owned())
                    .ok_or(CompilerError::LexicalError("Empty collective".into()))
            } else {
                Ok(Phrase::Primary(Primitive::Collective(phrases.into())))
            }
        }
    }
}

fn handle_postfix<'a, Buffer>(tokens : &mut Buffer, noun: Phrase) -> Result<Phrase, CompilerError> 
    where Buffer: TokenBuffer + Iterator<Item = &'a Token> {
    let token = match tokens.get_current() {
        Some(token) => token.to_owned(),
        None => {
            return Err(CompilerError::LexicalError("Unexpected EOF".into()));
        }
    };

    if tokens.match_next(&[TokenType::When]) {
        if let Precedent::Postfix(r_bp) = token.name.precedent() {
            let adjective = handle_adjective(tokens, r_bp)?;
            
            return Ok(Phrase::Postfix {
                noun: Box::new(noun),
                adjective: Box::new(adjective),
            });
        } else { 
            let msg = format!("[line {}] Error at '{}': {} has a wrong precedent type.", token.line, token.lexeme, token.name);
            return Err(CompilerError::LexicalError(msg.into()))
        }
    }

    tokens.advance();
    let msg = format!("[line {}] Error at '{}': {} is invalid postfix operator.", token.line, token.lexeme, token.name);
    Err(CompilerError::LexicalError(msg.into()))
}

fn handle_prefix<'a, Buffer>(tokens : &mut Buffer, token: Token) -> Result<Phrase, CompilerError> 
    where Buffer: TokenBuffer + Iterator<Item = &'a Token> {
    if let Precedent::Prefix(bp) = token.name.precedent() {
        let prefix = match token.name {
            TokenType::Not => Prefix::Not,
            TokenType::Minus => Prefix::Negation,
            TokenType::The => {
                let prefix_token = tokens.consume(TokenType::Identifier)?;
                Prefix::Adjective(prefix_token.lexeme.clone())
            },
            _ => Prefix::None,
        };

        if prefix == Prefix::None {
            let msg = format!("[line {}] Error at '{}': {} is unrecognised prefix operator.", token.line, token.lexeme, token.name);
            return Err(CompilerError::LexicalError(msg.into()))
        }
        
        let phrase = handle_phrase(tokens, bp)?;
        Ok(Phrase::Prefix {
            prefix,
            noun: Box::new(phrase),
        })
    } else {
        let msg = format!("[line {}] Error at '{}': {} is invalid prefix operator.", token.line, token.lexeme, token.name);
        Err(CompilerError::LexicalError(msg.into()))
    }
}

fn handle_adjective<'a, Buffer>(tokens : &mut Buffer, precedent: u8) -> Result<Phrase, CompilerError>
    where Buffer: TokenBuffer + Iterator<Item = &'a Token> {
    let category = tokens.next().map(|t| TokenCategory::from(t.to_owned()));

    let mut phrase = match category {
        Some(TokenCategory::Atom(token)) => handle_atom(token)?,
        Some(TokenCategory::Op(_)) => {
            return Err(CompilerError::LexicalError("Unsupported adjective as prefix".into()));
        },
        Some(TokenCategory::EOF) => {
            return Err(CompilerError::LexicalError("Unexpected EOF".into()));
        },
        None => {
            return Err(CompilerError::LexicalError("Unexpected EOF".into()));
        }
    };

    loop {
        // Check if there is an operator after the adjective, break the loop otherwise
        let op = match tokens.get_current().map(|t| TokenCategory::from(t.to_owned())) {
            Some(TokenCategory::Op(token)) => token,
            Some(TokenCategory::Atom(token @ Token { name: TokenType::Identifier, .. })) => token,
            Some(TokenCategory::Atom(token)) => {
                let msg = format!("[line {}] Error at '{}': {} is invalid operator.", token.line, token.lexeme, token.name);
                return Err(CompilerError::LexicalError(msg.into()));
            },
            Some(TokenCategory::EOF) => break,
            None => break,
        };

        match op.name.precedent() {
            Precedent::Infix(l_bp, r_bp) => {
                if l_bp < precedent { break; }
                tokens.next();

                let object = handle_adjective(tokens, r_bp)?;
    
                phrase = Phrase::Condition {
                    left: Box::new(phrase),
                    conjunction: op.name.into(),
                    right: Box::new(object),
                };

                continue;
            },
            _ => { break; },
        };
    }

    Ok(phrase)
}

fn handle_atom(token : Token) -> Result<Phrase, CompilerError> {
    if token.name == TokenType::It {
        return Ok(Phrase::Primary(Primitive::It));
    }

    if token.name == TokenType::False {
        return Ok(Phrase::Primary(Primitive::False));
    }
    if token.name == TokenType::True {
        return Ok(Phrase::Primary(Primitive::True));
    }

    if token.name == TokenType::Number {
        return Ok(Phrase::Primary(Primitive::Number(token.lexeme)));
    }
    if token.name == TokenType::Text {
        return Ok(Phrase::Primary(Primitive::Text(token.lexeme)));
    }
    if token.name == TokenType::Identifier {
        return Ok(Phrase::Primary(Primitive::Variable(token.lexeme)));
    }

    let msg = format!("At '{}' [line {}], invalid noun or adjective", token.lexeme, token.line);
    Err(CompilerError::LexicalError(msg.into()))
}
