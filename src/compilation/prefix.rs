use std::fmt;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Prefix {
    None,
    Not,
    Negation,
    Adjective(Rc<str>),
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Prefix::None => write!(f, "{:?}", self),
            Prefix::Not => write!(f, "!"),
            Prefix::Negation => write!(f, "-"),
            Prefix::Adjective(adj) => write!(f, "{adj}"),
        }
    }
}
