use std::fmt;

use crate::compilation::primitive::Primitive;
use crate::compilation::conjunction::Conjunction;
use crate::compilation::prefix::Prefix;
use crate::compilation::verb::Verb;


#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Phrase {
    None,
    Primary(Primitive),
    Postfix {
        noun: Box<Phrase>,
        adjective: Box<Phrase>,
    },
    Prefix {
        prefix: Prefix,
        noun: Box<Phrase>,
    },
    Action {
        subject: Option<Box<Phrase>>,
        verb: Verb,
        object: Option<Box<Phrase>>,
    },
    Condition {
        left: Box<Phrase>,
        conjunction: Conjunction,
        right: Box<Phrase>,
    },
}

impl fmt::Display for Phrase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Phrase::None => write!(f, "{self:?}"),
            Phrase::Primary(primitive) => write!(f, "{primitive}"),
            Phrase::Postfix { noun, adjective } => write!(f, "({noun} when {adjective})"),
            Phrase::Prefix { prefix, noun } => write!(f, "(prefix {prefix} {noun})"),
            Phrase::Action { subject: Some(sub), verb, object: Some(obj) } => write!(f, "{sub} {verb} {obj}"),
            Phrase::Action { subject, verb, object } => write!(f, "{subject:?} {verb} {object:?}"),
            Phrase::Condition { left, conjunction, right } => write!(f, "(condition {left} {conjunction} {right})"),
        }
    }
}
