use std::cell::RefCell;
use std::rc::Rc;

use crate::compilation::environment::Environment;

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Substantive {
    pub name: Rc<str>,
    pub environment: Rc<RefCell<Environment>>,
}

impl Substantive {
    pub fn new(name: &str, environment: Rc<RefCell<Environment>>) -> Self {
        Self {
            name: name.into(),
            environment,
        }
    }
}
