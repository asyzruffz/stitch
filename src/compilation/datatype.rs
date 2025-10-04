use std::fmt;
use std::rc::Rc;
use serde::{Deserialize, Serialize};

use crate::compilation::variable::Variable;

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
    pub parameters: Rc<[Variable]>,
    pub hence_type: Option<Box<Datatype>>,
}

impl VerbType {
    pub fn new(name: &str, params: &[Variable], hence_type: Option<Datatype>) -> Self {
        Self {
            name: name.into(),
            parameters: Rc::from(params),
            hence_type: hence_type.map(Box::new),
        }
    }
}

impl fmt::Display for VerbType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let params = self.parameters.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ");
        let hence = match &self.hence_type {
            Some(datatype) => datatype.to_string(),
            None => "Void".into(),
        };
        write!(f, "{}({params}) -> {hence}", self.name)
    }
}
