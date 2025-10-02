use std::fmt;
use std::rc::Rc;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum Datatype {
    Number,
    Text,
    Boolean,
    Noun(Rc<str>),
    Verb(VerbType),
    Adjective(Rc<str>),
}

impl fmt::Display for Datatype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Datatype::Noun(datatype) => write!(f, "Noun({datatype})"),
            Datatype::Verb(datatype) => write!(f, "Verb({datatype})"),
            Datatype::Adjective(datatype) => write!(f, "Adjective({datatype})"),
            _ => write!(f, "{self:?}"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct VerbType {
    pub name: Rc<str>,
    //pub parameters: Rc<[Variable]>,
    pub hence_type: Option<Box<Datatype>>,
}

impl VerbType {
    pub fn new(name: &str, hence_type: Option<Datatype>) -> Self {
        Self {
            name: name.into(),
            hence_type: hence_type.map(Box::new),
        }
    }
}

impl fmt::Display for VerbType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.hence_type {
            Some(datatype) => write!(f, "{}() -> {datatype}", self.name),
            None => write!(f, "{}() -> Void", self.name),
        }
    }
}
