use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::compilation::evaluation::Evaluation;
use crate::compilation::variable::Variable;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compilation::datatype::Datatype;

    #[test]
    fn define_and_get() {
        let mut env = Environment::default();
        let var = Variable::new("x", &Datatype::Number);
        let eval = Evaluation::Number(42.0);

        env.define(var.clone(), eval.clone());
        assert_eq!(env.get("x"), Some(eval));
    }

    #[test]
    fn assign_existing_variable() {
        let mut env = Environment::default();
        let var = Variable::new("x", &Datatype::Number);
        let eval1 = Evaluation::Number(42.0);
        let eval2 = Evaluation::Number(100.0);

        env.define(var.clone(), eval1);
        assert!(env.assign(var.clone(), eval2.clone()).is_ok());
        assert_eq!(env.get("x"), Some(eval2));
    }

    #[test]
    fn assign_nonexistent_variable() {
        let mut env = Environment::default();
        let var = Variable::new("x", &Datatype::Number);
        let eval = Evaluation::Number(42.0);

        assert!(env.assign(var.clone(), eval).is_err());
    }

    #[test]
    fn nested_environment() {
        let outer_env = Rc::new(RefCell::new(Environment::default()));
        let mut inner_env = Environment::within_scope(outer_env.clone());

        let outer_var = Variable::new("y", &Datatype::Number);
        let outer_eval = Evaluation::Number(10.0);
        outer_env.borrow_mut().define(outer_var.clone(), outer_eval.clone());

        let inner_var = Variable::new("x", &Datatype::Number);
        let inner_eval = Evaluation::Number(20.0);
        inner_env.define(inner_var.clone(), inner_eval.clone());

        assert_eq!(inner_env.get("x"), Some(inner_eval));
        assert_eq!(inner_env.get("y"), Some(outer_eval));
        assert_eq!(outer_env.borrow().get("x"), None);
    }
}
