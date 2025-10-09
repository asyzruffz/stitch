use std::fmt;
use std::rc::Rc;

use crate::compilation::datatype::{Datatype, VerbType};
use crate::compilation::routine::Routine;
use crate::compilation::substantive::Substantive;

#[derive(Default, PartialEq, Clone, Debug)]
pub enum Evaluation {
    #[default] Void,
    Skip(Box<Evaluation>),
    Conclusion(Box<Evaluation>),
    Number(f32),
    Text(Rc<str>),
    Notion(bool),
    Collective(Rc<[Evaluation]>),
    Noun(Substantive),
    Action(Routine),
    Adjective(Routine),
}

impl fmt::Display for Evaluation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Evaluation::Void => write!(f, "void"),
            Evaluation::Skip(eval) => write!(f, "skip ({})", eval.as_ref()),
            Evaluation::Conclusion(eval) => write!(f, "conclusion ({})", eval.as_ref()),
            Evaluation::Number(value) => write!(f, "{}", value),
            Evaluation::Text(value) => write!(f, "{}", value),
            Evaluation::Notion(value) => write!(f, "{}", value),
            Evaluation::Collective(evaluations) => write!(f, "{}", evaluations.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ")),
            Evaluation::Noun(substantive) => write!(f, "{}", substantive.name),
            Evaluation::Action(routine) => write!(f, "{}()", routine.name),
            Evaluation::Adjective(routine) => write!(f, "{}<>", routine.name),
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
            (Evaluation::Notion(lval), Evaluation::Notion(rval)) => lval == rval,
            // TODO: Implement custom equality for Noun, Action, Adjective
            _ => false,
        }
    }

    pub fn datatype(&self) -> Option<Datatype> {
        match self {
            Evaluation::Void => None,
            Evaluation::Skip(_) => None,
            Evaluation::Conclusion(_) => None,
            Evaluation::Number(_) => Some(Datatype::Number),
            Evaluation::Text(_) => Some(Datatype::Text),
            Evaluation::Notion(_) => Some(Datatype::Notion),
            Evaluation::Collective(_) => None,
            Evaluation::Noun(substantive) => Some(Datatype::Noun(substantive.name.clone())),
            Evaluation::Action(routine) => {
                let variables = routine.object_parameters.iter().map(|param| param.variable.clone()).collect::<Vec<_>>();
                Some(Datatype::Verb(VerbType::new(routine.name.as_ref(), variables.as_slice(), routine.subject_type.clone())))
            },
            Evaluation::Adjective(routine) => Some(Datatype::Adjective(routine.name.clone())),
        }
    }

    pub fn parity(&self, other: &Evaluation) -> Result<(), Rc<str>> {
        match (self, other) {
            (Evaluation::Void, Evaluation::Void) => Ok(()),
            (Evaluation::Void, found) => match found.datatype() {
                None => Ok(()),
                Some(datatype) => Err(format!("expected Void but found {}", datatype).into()),
            },
            (expected, Evaluation::Void) => match expected.datatype() {
                None => Ok(()),
                Some(datatype) => Err(format!("expected {} but found Void", datatype).into()),
            },
            (Evaluation::Collective(expected), Evaluation::Collective(found)) => {
                if expected.len() != found.len() {
                    return Err(format!("collectives do not match in length, expected {} but found {} object(s)", expected.len(), found.len()).into());
                }

                expected.iter()
                        .zip(found.iter())
                        .map(|(e1, e2)| e1.parity(e2))
                        .collect::<Result<Vec<_>, Rc<str>>>()
                        .map(|_| ())
            },
            (Evaluation::Collective(expected), found) => {
                if expected.len() == 1 {
                    expected[0].parity(found)
                } else {
                    Err(format!("collectives do not match in length, expected {} but found 1 object(s)", expected.len()).into())
                }
            },
            (expected, Evaluation::Collective(found)) => {
                if found.len() == 1 {
                    expected.parity(&found[0])
                } else {
                    Err(format!("collectives do not match in length, expected 1 but found {} object(s)", found.len()).into())
                }
            },
            (Evaluation::Number(_), Evaluation::Number(_)) => Ok(()),
            (Evaluation::Text(_), Evaluation::Text(_)) => Ok(()),
            (Evaluation::Notion(_), Evaluation::Notion(_)) => Ok(()),
            (Evaluation::Noun(expected), Evaluation::Noun(found)) => if expected.name.as_ref() == found.name.as_ref() {
                Ok(())
            } else {
                Err(format!("expected {} but found {}", expected.name, found.name).into())
            },
            (expected, found) => { // TODO: Handle parity for Action, Adjective
                let expected_type = expected.datatype().map_or("Void".into(), |dt| format!("{dt}"));
                let found_type = found.datatype().map_or("Void".into(), |dt| format!("{dt}"));
                Err(format!("expected {} but found {}", expected_type, found_type).into())
            },
        }
    }
}
