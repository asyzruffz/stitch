use std::fmt;
use std::rc::Rc;

#[derive(Default, PartialEq, Clone, Debug)]
pub enum Evaluation {
    #[default] Void,
    Number(f32),
    Text(Rc<str>),
    Boolean(bool),
    Collective(Rc<[Evaluation]>),
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
}
