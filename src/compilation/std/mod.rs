//mod io;

use std::cell::RefCell;
use std::rc::Rc;

use crate::compilation::environment::Environment;
//use crate::compilation::std::io::add_print;

pub fn add_builtin_features(environment: Rc<RefCell<Environment>>) {
    //add_print(environment.clone());
}
