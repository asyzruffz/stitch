use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::compilation::datatype::Datatype;
use crate::compilation::evaluation::Evaluation;

#[derive(Default, Debug)]
pub struct Environment {
    pub outer: Option<Box<Environment>>,
    values: HashMap<Variable, Evaluation>,
}

impl Environment {
    pub fn within_scope(outer: Environment) -> Self {
        Self {
            outer: Some(Box::new(outer)),
            ..Default::default()
        }
    }
    
    pub fn define(&mut self, var: Variable, value: Evaluation) {
        self.values.insert(var, value);
    }

    pub fn assign(&mut self, var: Variable, value: Evaluation) -> Result<(), String> {
        if self.contains_var(&var) {
            self.values.insert(var, value);
            Ok(())
        } else if let Some(env) = self.outer.as_mut() {
            env.assign(var, value)
        } else {
            Err(var.name)
        }
    }

    pub fn get(&self, name: &str) -> Option<&Evaluation> {
        self.values.get(&Variable::with(name))
            .or_else(|| self.outer.as_ref()
                .and_then(|env| env.get(name)))
    }

    pub fn contains_var(&self, var: &Variable) -> bool {
        self.values.contains_key(var)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Variable {
    pub name: String,
    pub datatype: Option<Datatype>,
}

impl Hash for Variable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
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
