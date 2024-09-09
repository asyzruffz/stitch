use std::fmt;
use std::rc::Rc;

use crate::compilation::expression::Expression;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Statement {
    Print(Expression),
    Expression(Expression),
    Var {
        name: Rc<str>,
        initializer: Option<Expression>,
    },
    Block(Vec<Statement>),
    If {
        condition: Expression,
        then_statement: Box<Statement>,
        else_statement: Option<Box<Statement>>,
    },
    While {
        condition: Expression,
        body_statement: Box<Statement>,
    },
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Print(printable) => write!(f, "print {printable}"),
            Statement::Expression(expr) => write!(f, "{expr}"),
            Statement::Var { name, initializer } => write!(f, "{name} = {initializer:?}"),
            Statement::Block(stmnts) => write!(f, "{{{}\n}}", stmnts.iter().map(|s| s.to_string()).collect::<Vec<_>>().join("\n\t")),
            Statement::If { condition, then_statement, else_statement } => write!(f, "if {condition} then {then_statement} else {else_statement:?})"),
            Statement::While { condition, body_statement } => write!(f, "while {condition} {body_statement})"),
        }
    }
}
