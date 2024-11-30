use std::fmt;
use std::rc::Rc;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum Datatype {
    Number,
    Text,
    Boolean,
    Custom(Rc<str>),
}

impl fmt::Display for Datatype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Datatype::Custom(datatype) => write!(f, "{datatype}"),
            _ => write!(f, "{self:?}"),
        }
    }
}
