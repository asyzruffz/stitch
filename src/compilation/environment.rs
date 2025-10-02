use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use derivative::Derivative;

use crate::compilation::datatype::Datatype;
use crate::compilation::evaluation::Evaluation;

#[derive(Default, PartialEq, Debug)]
pub struct Environment {
    pub outer: Option<Rc<RefCell<Environment>>>,
    values: HashMap<Variable, Evaluation>,
}

impl Environment {
    pub fn within_scope<'outer>(outer: Rc<RefCell<Environment>>) -> Self {
        Self {
            outer: Some(outer),
            ..Default::default()
        }
    }
    
    pub fn define(&mut self, var: Variable, value: Evaluation) {
        self.values.insert(var, value);
    }

    pub fn assign(&mut self, var: Variable, value: Evaluation) -> Result<(), String> {
        if self.contains_var(&var.name) {
            self.values.insert(var, value);
            Ok(())
        } else if let Some(env) = self.outer.as_mut() {
            env.borrow_mut().assign(var, value)
        } else {
            Err(var.name)
        }
    }

    pub fn get(&self, name: &str) -> Option<Evaluation> {
        self.values.get(&Variable::with(name)).cloned()
            .or_else(|| self.outer.as_ref()
                .and_then(|env| env.borrow().get(name)))
    }

    pub fn contains_var(&self, name: &str) -> bool {
        self.values.contains_key(&Variable::with(name))
    }

    fn display(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        if let Some(outer) = self.outer.clone() {
            outer.borrow().display(f, indent + 2)?;
        }
        for (var, eval) in &self.values {
            writeln!(f, "{}{} = {}", " ".repeat(indent), var, eval)?;
        }
        Ok(())
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Environment {{")?;
        self.display(f, 2)?;
        writeln!(f, "}}")?;
        Ok(())
    }
}

#[derive(Derivative, Clone, Debug)]
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
