use std::{fmt, str};
use std::rc::Rc;

use crate::compilation::datatype::Datatype;
use crate::compilation::phrase::Phrase;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Statement {
    Noun {
        name: Rc<str>,
        super_type: Option<Datatype>,
        body: Statements,
    },
    Verb {
        name: Rc<str>,
        hence_type: Option<Datatype>,
        subject_type: Option<Datatype>,
        object_types: Rc<[Statement]>,
        body: Statements,
    },
    Adjective {
        name: Rc<str>,
        subject_type: Datatype,
        body: Statements,
    },
    So {
        name: Rc<str>,
        datatype: Datatype,
        initializer: Option<Phrase>,
    },
    Phrase(Phrase),
    Hence(Phrase),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Noun { name, super_type: Some(supertype), body } => write!(f, "noun {name} is {supertype} \n{body}"),
            Statement::Noun { name, super_type, body } => write!(f, "noun {name} is {super_type:?} \n{body}"),
            Statement::Verb { name, hence_type: Some(hencetype), subject_type: Some(subjecttype), object_types, body } => 
                write!(f, "verb {name} is {hencetype} for {subjecttype} when {} \n{body}",
                    object_types.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")),
            Statement::Verb { name, hence_type, subject_type, object_types, body } => 
                write!(f, "verb {name} is {hence_type:?} for {subject_type:?} when {} \n{body}",
                    object_types.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")),
            Statement::Adjective { name, subject_type, body } => write!(f, "adjective {name} for {subject_type} \n{body}"),
            Statement::So { name, datatype, initializer: Some(init) } => write!(f, "so {name} is {datatype} as {init}"),
            Statement::So { name, datatype, .. } => write!(f, "so {name} is {datatype}"),
            Statement::Phrase(phrase) => write!(f, "{phrase}."),
            Statement::Hence(phrase) => write!(f, "hence {phrase}."),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Statements(pub Rc<[Statement]>);

impl fmt::Display for Statements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\n    {}\n}}", self.0.as_ref().iter().map(|s| s.to_string()).collect::<Vec<_>>().join("\n    "))
    }
}
