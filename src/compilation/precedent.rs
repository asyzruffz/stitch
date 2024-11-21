use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Precedent {
    None,
    Prefix(u8),
    Postfix(u8),
    Infix(u8, u8),
}

impl fmt::Display for Precedent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Precedent::None => write!(f, "(None)"),
            Precedent::Prefix(r_bp) => write!(f, "(-, {})", r_bp),
            Precedent::Postfix(l_bp) => write!(f, "({}, -)", l_bp),
            Precedent::Infix(l_bp, r_bp) => write!(f, "({}, {})", l_bp, r_bp),
        }
    }
}
