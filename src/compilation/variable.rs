use std::fmt;

use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::compilation::{datatype::Datatype, evaluation::Evaluation};

#[derive(Derivative, Clone, Debug, Deserialize, Serialize)]
#[derivative(PartialEq, Eq, Hash)]
pub struct Variable {
    pub name: String,
    #[derivative(PartialEq="ignore")]
    #[derivative(Hash="ignore")]
    pub datatype: Option<Datatype>,
}

impl Variable {
    pub fn new(name: &str, datatype: &Datatype) -> Self {
        Self {
            name: name.to_string(),
            datatype: Some(datatype.clone())
        }
    }

    pub fn with(name: &str) -> Self {
        Self {
            name: name.to_string(),
            datatype: None
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Variable { name, datatype: Some(datatype) } => write!(f, "{}: {}", name, datatype),
            Variable { name, datatype: None } => write!(f, "{}", name),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct VariableValue {
    pub variable: Variable,
    pub value: Evaluation,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        let var1 = Variable::new("x", &Datatype::Number);
        let var2 = Variable::new("x", &Datatype::Text);
        let var3 = Variable::with("x");
        let var4 = Variable::with("y");

        assert_eq!(var1, Variable::new("x", &Datatype::Number));
        assert_eq!(var1, var2);
        assert_eq!(var1, var3);
        assert_ne!(var1, var4);
    }
}
