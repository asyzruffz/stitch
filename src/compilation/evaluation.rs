use std::fmt;
use std::rc::Rc;

use crate::compilation::datatype::Datatype;
use crate::compilation::routine::Routine;
use crate::compilation::substantive::Substantive;

#[derive(Default, PartialEq, Clone, Debug)]
pub enum Evaluation {
    #[default] Void,
    Number(f32),
    Text(Rc<str>),
    Boolean(bool),
    Collective(Rc<[Evaluation]>),
    Noun(Substantive),
    Action(Routine),
    Custom(Rc<str>),
}

impl fmt::Display for Evaluation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Evaluation::Void => write!(f, "void"),
            Evaluation::Number(value) => write!(f, "{}", value),
            Evaluation::Text(value) => write!(f, "{}", value),
            Evaluation::Boolean(value) => write!(f, "{}", value),
            Evaluation::Collective(evaluations) => write!(f, "{}", evaluations.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ")),
            Evaluation::Noun(substantive) => write!(f, "{}", substantive.name),
            Evaluation::Action(routine) => write!(f, "{}()", routine.name),
            Evaluation::Custom(typename) => write!(f, "{} {{..}}", typename),
        }
    }
}

impl Evaluation {
    pub fn is_number(&self) -> bool {
        if let Evaluation::Number(_) = self { true }
        else { false }
    }

    pub fn equal(left: &Evaluation, right: &Evaluation) -> bool {
        match (left, right) {
            (Evaluation::Void, Evaluation::Void) => true,
            (Evaluation::Void, _) => false,
            (_, Evaluation::Void) => false,
            (Evaluation::Number(lval), Evaluation::Number(rval)) => (lval - rval).abs() < f32::EPSILON,
            (Evaluation::Text(lval), Evaluation::Text(rval)) => lval.as_ref() == rval.as_ref(),
            (Evaluation::Boolean(lval), Evaluation::Boolean(rval)) => lval == rval,
            (Evaluation::Custom(_), Evaluation::Custom(_)) => todo!(),
            _ => false,
        }
    }

    pub fn datatype(&self) -> Option<Datatype> {
        match self {
            Evaluation::Void => None,
            Evaluation::Number(_) => Some(Datatype::Number),
            Evaluation::Text(_) => Some(Datatype::Text),
            Evaluation::Boolean(_) => Some(Datatype::Boolean),
            Evaluation::Collective(_) => None,
            Evaluation::Noun(_) => None,
            Evaluation::Action(_) => None,
            Evaluation::Custom(typename) => Some(Datatype::Custom(typename.clone())),
        }
    }

    pub fn parity(&self, other: &Evaluation) -> bool {
        match (self, other) {
            (Evaluation::Void, Evaluation::Void) => true,
            (Evaluation::Void, _) => false,
            (_, Evaluation::Void) => false,
            (Evaluation::Number(_), Evaluation::Number(_)) => true,
            (Evaluation::Text(_), Evaluation::Text(_)) => true,
            (Evaluation::Boolean(_), Evaluation::Boolean(_)) => true,
            (Evaluation::Custom(typename1), Evaluation::Custom(typename2)) => typename1 == typename2,
            (Evaluation::Collective(evaluations1), Evaluation::Collective(evaluations2)) => 
                evaluations1.len() == evaluations2.len() && evaluations1.iter().zip(evaluations2.iter()).all(|(e1, e2)| e1.parity(e2)),
            _ => false,
        }
    }
}
