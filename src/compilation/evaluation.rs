use std::fmt;
use std::rc::Rc;

#[derive(Default, PartialEq, Clone, Debug)]
pub enum Evaluation {
    #[default] Void,
    Number(f32),
    Text(Rc<str>),
    Boolean(bool),
    Custom(Rc<str>),
}

impl fmt::Display for Evaluation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Evaluation::Void => write!(f, "void"),
            Evaluation::Number(value) => write!(f, "{}", value),
            Evaluation::Text(value) => write!(f, "{}", value),
            Evaluation::Boolean(value) => write!(f, "{}", value),
            Evaluation::Custom(typename) => write!(f, "{} {{..}}", typename),
        }
    }
}
