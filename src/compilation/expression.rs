use std::fmt;
use std::rc::Rc;

use crate::compilation::operator::Operator;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Expression {
    None,
    Call{
        callee: Box<Expression>,
        arguments: Rc<[Expression]>,
    },
    Primary(LiteralExpression),
    Unary {
        operator: Operator,
        right: Box<Expression>,
    },
    Binary {
        operator: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::None => write!(f, "{:?}", self),
            Expression::Call{ callee, arguments } => write!(f, "(call {} {:?})", callee, arguments),
            Expression::Primary(expr) => write!(f, "{}", expr),
            Expression::Unary { operator, right } => write!(f, "({} {})", operator, right),
            Expression::Binary { operator, right, left } => write!(f, "({} {} {})", operator, left, right),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum LiteralExpression {
    Number(Rc<str>),
    String(Rc<str>),
    True,
    False,
    Nil,
    Group(Box<Expression>),
    Variable(Rc<str>),
}

impl fmt::Display for LiteralExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralExpression::Number(literal) => write!(f, "{literal}"),
            LiteralExpression::String(literal) => write!(f, "{literal}"),
            LiteralExpression::True => write!(f, "true"),
            LiteralExpression::False => write!(f, "false"),
            LiteralExpression::Nil => write!(f, "nil"),
            LiteralExpression::Group(expr) => write!(f, "(group {expr})"),
            LiteralExpression::Variable(name) => write!(f, "(var {})", name),
        }
    }
}
