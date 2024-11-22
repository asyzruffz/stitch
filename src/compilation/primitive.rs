use std::fmt;
use std::rc::Rc;

use crate::compilation::datatype::Datatype;
use crate::compilation::phrase::Phrase;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Primitive {
    Type(Datatype),
    Number(Rc<str>),
    Text(Rc<str>),
    True,
    False,
    It,
    Variable(Rc<str>),
    Collective(Rc<[Phrase]>),
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Primitive::Type(datatype) => write!(f, "{datatype}"),
            Primitive::Number(literal) => write!(f, "{literal}"),
            Primitive::Text(literal) => write!(f, "{literal}"),
            Primitive::True => write!(f, "true"),
            Primitive::False => write!(f, "false"),
            Primitive::It => write!(f, "it"),
            Primitive::Variable(name) => write!(f, "{name}"),
            Primitive::Collective(exprs) => write!(f, "({})", exprs.as_ref()
                .iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ")),
        }
    }
}
