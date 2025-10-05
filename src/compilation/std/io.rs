use std::cell::RefCell;
use std::rc::Rc;

use crate::compilation::datatype::{Datatype, VerbType};
use crate::compilation::environment::Environment;
use crate::compilation::errors::EvaluationError;
use crate::compilation::evaluation::Evaluation;
use crate::compilation::routine::Routine;
use crate::compilation::variable::{Variable, VariableValue};

pub fn add_print(environment: Rc<RefCell<Environment>>) {
    let name = "print";
    let params = vec![
        VariableValue { variable: Variable::new("value", &Datatype::Text), value: Evaluation::Text("".into()) },
    ];

    let param_vars = params.iter().map(|v| v.variable.clone()).collect::<Vec<_>>();
    let variable = Variable::new(name, &Datatype::Verb(VerbType::new(name, param_vars.as_ref(), None)));

    let subject_type = Some(Datatype::Number);
    let action = Evaluation::Action(Routine::new_native(name, subject_type, params.into(), print_fn));
    
    environment.borrow_mut().define(variable, action);

    let name = "printn";
    let params = vec![
        VariableValue { variable: Variable::new("value", &Datatype::Number), value: Evaluation::Number(0.0) },
    ];

    let param_vars = params.iter().map(|v| v.variable.clone()).collect::<Vec<_>>();
    let variable = Variable::new(name, &Datatype::Verb(VerbType::new(name, param_vars.as_ref(), None)));

    let subject_type = Some(Datatype::Number);
    let action = Evaluation::Action(Routine::new_native(name, subject_type, params.into(), print_fn));
    
    environment.borrow_mut().define(variable, action);
}

fn print_fn(subj: Evaluation, objs: Evaluation) -> Result<Evaluation, EvaluationError> {
    println!("{objs}");
    Ok(subj)
}
