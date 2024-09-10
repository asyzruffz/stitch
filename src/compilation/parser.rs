use std::rc::Rc;

use crate::compilation::token::{Token, TokenType, TokenBuffer};
use crate::compilation::expression::{Expression, LiteralExpression};
use crate::compilation::operator::Operator;
use crate::compilation::statement::Statement;
use crate::compilation::errors::CompilerError;

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
            match handle_declaration(&mut buffer) {
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

fn handle_declaration(tokens : &mut impl TokenBuffer) -> Result<Statement, CompilerError> {
    if tokens.match_next(&[TokenType::Var]) {
        return handle_var_declaration(tokens);
    }

    handle_statement(tokens)
}

fn handle_var_declaration(tokens : &mut impl TokenBuffer) -> Result<Statement, CompilerError> {
    let token = match tokens.consume(TokenType::Identifier) {
        Ok(token) => token.to_owned(),
        Err(error) => return Err(error),
    };

    let initializer = if tokens.match_next(&[TokenType::Equal]) {
        Some(handle_expression(tokens)?)
    } else { None };

    if let Err(error) = tokens.consume(TokenType::Semicolon) {
        return Err(error);
    }

    Ok(Statement::Var { name: token.lexeme, initializer })
}

fn handle_statement(tokens : &mut impl TokenBuffer) -> Result<Statement, CompilerError> {
    /*if tokens.match_next(&[TokenType::For]) {
        handle_for_statement(tokens)
    } else if tokens.match_next(&[TokenType::If]) {
        handle_if_statement(tokens)
    } else if tokens.match_next(&[TokenType::Print]) {
        handle_print_statement(tokens)
    } else if tokens.match_next(&[TokenType::While]) {
        handle_while_statement(tokens)
    } else*/ if tokens.match_next(&[TokenType::LeftBrace]) {
        handle_block_statement(tokens)
    } else {
        handle_expression_statement(tokens)
    }
}

fn handle_block_statement(tokens : &mut impl TokenBuffer) -> Result<Statement, CompilerError> {
    let mut statements = Vec::new();

    while !tokens.peek_next(TokenType::RightBrace) && !tokens.is_at_end() {
        let statement = handle_declaration(tokens)?;
        statements.push(statement);
    }

    if let Err(error) = tokens.consume(TokenType::RightBrace) {
        return Err(error);
    }

    Ok(Statement::Block(statements))
}

fn handle_expression_statement(tokens : &mut impl TokenBuffer) -> Result<Statement, CompilerError> {
    let expr = handle_expression(tokens)?;

    if let Err(error) = tokens.consume(TokenType::Semicolon) {
        return Err(error);
    }

    Ok(Statement::Expression(expr))
}

fn handle_expression(tokens : &mut impl TokenBuffer) -> Result<Expression, CompilerError> {
    handle_assignment(tokens)
}

fn handle_assignment(tokens : &mut impl TokenBuffer) -> Result<Expression, CompilerError> {
    let expr = handle_or(tokens)?;

    let token = match tokens.get_current() {
        Some(token) => token.to_owned(),
        None => {
            return Err(CompilerError::LexicalError("Unexpected EOF".into()));
        }
    };

    if tokens.match_next(&[TokenType::Equal]) {
        let value = handle_assignment(tokens)?;
        
        if let Expression::Primary(LiteralExpression::Variable(_)) = expr {
            return Ok(Expression::Binary {
                left: Box::new(expr),
                operator: Operator::Assignment,
                right: Box::new(value),
            });
        } else {
            tokens.advance();
            let msg = format!("[line {}] Error at '{}': Invalid assignment target.", token.line, token.lexeme);
            return Err(CompilerError::LexicalError(msg.into()));
        }
    }

    Ok(expr)
}

fn handle_or(tokens : &mut impl TokenBuffer) -> Result<Expression, CompilerError> {
    let mut expr = handle_and(tokens)?;

    let mut token = match tokens.get_current() {
        Some(token) => token.to_owned(),
        None => {
            return Err(CompilerError::LexicalError("Unexpected EOF".into()));
        }
    };

    while tokens.match_next(&[TokenType::Or]) {
        let operator = token;
        let right = handle_and(tokens)?;
        expr = Expression::Binary {
            left: Box::new(expr),
            operator: Operator::from(operator.name),
            right: Box::new(right),
        };

        token = match tokens.get_current() {
            Some(token) => token.to_owned(),
            None => {
                return Err(CompilerError::LexicalError("Unexpected EOF".into()));
            }
        };
    }

    Ok(expr)
}

fn handle_and(tokens : &mut impl TokenBuffer) -> Result<Expression, CompilerError> {
    let mut expr = handle_equality(tokens)?;

    let mut token = match tokens.get_current() {
        Some(token) => token.to_owned(),
        None => {
            return Err(CompilerError::LexicalError("Unexpected EOF".into()));
        }
    };

    while tokens.match_next(&[TokenType::And]) {
        let operator = token;
        let right = handle_equality(tokens)?;
        expr = Expression::Binary {
            left: Box::new(expr),
            operator: Operator::from(operator.name),
            right: Box::new(right),
        };

        token = match tokens.get_current() {
            Some(token) => token.to_owned(),
            None => {
                return Err(CompilerError::LexicalError("Unexpected EOF".into()));
            }
        };
    }

    Ok(expr)
}

fn handle_equality(tokens : &mut impl TokenBuffer) -> Result<Expression, CompilerError> {
    let mut expr = handle_comparison(tokens)?;

    let mut token = match tokens.get_current() {
        Some(token) => token.to_owned(),
        None => {
            return Err(CompilerError::LexicalError("Unexpected EOF".into()));
        }
    };

    while tokens.match_next(&[TokenType::BangEqual, TokenType::EqualEqual]) {
        let operator = token;
        let right = handle_comparison(tokens)?;
        expr = Expression::Binary {
            left: Box::new(expr),
            operator: Operator::from(operator.name),
            right: Box::new(right),
        };

        token = match tokens.get_current() {
            Some(token) => token.to_owned(),
            None => {
                return Err(CompilerError::LexicalError("Unexpected EOF".into()));
            }
        };
    }

    Ok(expr)
}

fn handle_comparison(tokens : &mut impl TokenBuffer) -> Result<Expression, CompilerError> {
    let mut expr = handle_term(tokens)?;

    let mut token = match tokens.get_current() {
        Some(token) => token.to_owned(),
        None => {
            return Err(CompilerError::LexicalError("Unexpected EOF".into()));
        }
    };

    while tokens.match_next(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
        let operator = token;
        let right = handle_term(tokens)?;
        expr = Expression::Binary {
            left: Box::new(expr),
            operator: Operator::from(operator.name),
            right: Box::new(right),
        };

        token = match tokens.get_current() {
            Some(token) => token.to_owned(),
            None => {
                return Err(CompilerError::LexicalError("Unexpected EOF".into()));
            }
        };
    }

    Ok(expr)
}

fn handle_term(tokens : &mut impl TokenBuffer) -> Result<Expression, CompilerError> {
    let mut expr = handle_factor(tokens)?;

    let mut token = match tokens.get_current() {
        Some(token) => token.to_owned(),
        None => {
            return Err(CompilerError::LexicalError("Unexpected EOF".into()));
        }
    };

    while tokens.match_next(&[TokenType::Minus, TokenType::Plus]) {
        let operator = token;
        let right = handle_factor(tokens)?;
        expr = Expression::Binary {
            left: Box::new(expr),
            operator: Operator::from(operator.name),
            right: Box::new(right),
        };

        token = match tokens.get_current() {
            Some(token) => token.to_owned(),
            None => {
                return Err(CompilerError::LexicalError("Unexpected EOF".into()));
            }
        };
    }

    Ok(expr)
}

fn handle_factor(tokens : &mut impl TokenBuffer) -> Result<Expression, CompilerError> {
    let mut expr = handle_unary(tokens)?;

    let mut token = match tokens.get_current() {
        Some(token) => token.to_owned(),
        None => {
            return Err(CompilerError::LexicalError("Unexpected EOF".into()));
        }
    };

    while tokens.match_next(&[TokenType::Slash, TokenType::Star]) {
        let operator = token;
        let right = handle_unary(tokens)?;
        expr = Expression::Binary {
            left: Box::new(expr),
            operator: Operator::from(operator.name),
            right: Box::new(right),
        };

        token = match tokens.get_current() {
            Some(token) => token.to_owned(),
            None => {
                return Err(CompilerError::LexicalError("Unexpected EOF".into()));
            }
        };
    }

    Ok(expr)
}

fn handle_unary(tokens : &mut impl TokenBuffer) -> Result<Expression, CompilerError> {
    let token = match tokens.get_current() {
        Some(token) => token.to_owned(),
        None => {
            return Err(CompilerError::LexicalError("Unexpected EOF".into()));
        }
    };

    if tokens.match_next(&[TokenType::Bang, TokenType::Minus]) {
        let operator = token;
        let right = handle_unary(tokens)?;
        return Ok(Expression::Unary {
            operator: if operator.name == TokenType::Minus { Operator::Negation } else { Operator::from(operator.name) },
            right: Box::new(right),
        });
    }

    handle_call(tokens)
}

fn handle_call(tokens : &mut impl TokenBuffer) -> Result<Expression, CompilerError> {
    let mut expr = handle_primary(tokens)?;

    loop {
        if tokens.match_next(&[TokenType::LeftParen]) {
            let mut arguments = Vec::new();
            if !tokens.peek_next(TokenType::RightParen) {
                loop {
                    if arguments.len() >= 255 {
                        eprint!("Can't have more than 255 arguments.");
                    }

                    arguments.push(handle_expression(tokens)?);
                    if !tokens.match_next(&[TokenType::Comma]) {
                        break;
                    }
                }
            }

            if let Err(error) = tokens.consume(TokenType::RightParen) {
                return Err(error);
            }

            expr = Expression::Call {
                callee: Box::new(expr),
                arguments: arguments.into(),
            };
        } else {
            break;
        }
    }

    Ok(expr)
}

fn handle_primary(tokens : &mut impl TokenBuffer) -> Result<Expression, CompilerError> {
    if tokens.match_next(&[TokenType::False]) {
        return Ok(Expression::Primary(LiteralExpression::False));
    }
    if tokens.match_next(&[TokenType::True]) {
        return Ok(Expression::Primary(LiteralExpression::True));
    }
    if tokens.match_next(&[TokenType::Nil]) {
        return Ok(Expression::Primary(LiteralExpression::Nil));
    }

    let token = match tokens.get_current() {
        Some(token) => token.to_owned(),
        None => {
            return Err(CompilerError::LexicalError("Unexpected EOF".into()));
        }
    };
    
    if tokens.match_next(&[TokenType::Number]) {
        return Ok(Expression::Primary(LiteralExpression::Number(token.lexeme)));
    }
    if tokens.match_next(&[TokenType::String]) {
        return Ok(Expression::Primary(LiteralExpression::String(token.lexeme)));
    }
    if tokens.match_next(&[TokenType::Identifier]) {
        return Ok(Expression::Primary(LiteralExpression::Variable(token.lexeme)));
    }

    if tokens.match_next(&[TokenType::LeftParen]) {
        let expr = handle_expression(tokens)?;

        if let Err(error) = tokens.consume(TokenType::RightParen) {
            return Err(error);
        }

        return Ok(Expression::Primary(LiteralExpression::Group(Box::new(expr))));
    }

    tokens.advance();
    let msg = format!("[line {}] Error at '{}': Expect expression.", token.line, token.lexeme);
    Err(CompilerError::LexicalError(msg.into()))
}
