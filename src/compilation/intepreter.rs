use std::cell::RefCell;

use crate::compilation::datatype::Datatype;
use crate::compilation::environment::{Environment, Variable};
use crate::compilation::errors::EvaluationError;
use crate::compilation::evaluation::Evaluation;
use crate::compilation::phrase::Phrase;
use crate::compilation::primitive::Primitive;
use crate::compilation::statement::Statement;

#[derive(Default)]
pub struct Intepreter {
    environment: RefCell<Environment>,
}

impl Intepreter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn execute(&mut self, statement : &Statement) -> Result<Evaluation, EvaluationError> {
        match statement {
            Statement::Noun { name, super_type, body } => todo!(),
            Statement::Verb { name, hence_type, subject_type, object_types, body } => todo!(),
            Statement::Adjective { name, subject_type, body } => todo!(),
            Statement::So { name, datatype, initializer } => declare_so(name, datatype, initializer.as_ref(), &mut self.environment.borrow_mut()),
            Statement::Phrase(phrase) => evaluate(phrase, &mut self.environment.borrow_mut()),
            Statement::Hence(phrase) => todo!(),
        }
    }
}

fn declare_so(name: &str, datatype: &Datatype, initializer : Option<&Phrase>, environment: &mut Environment) -> Result<Evaluation, EvaluationError> {
    let variable = Variable::new(name, datatype);
    match initializer {
        None => {
            environment.define(variable, Evaluation::Void);
            Ok(Evaluation::Void)
        },
        Some(phrase) => {
            let result = evaluate(phrase, environment)?;
            match result {
                Evaluation::Void => Err(EvaluationError::new("Unable to initialize so declaration as void")),
                value => {
                    environment.define(variable, value);
                    Ok(Evaluation::Void)
                },
            }
        },
    }
}

fn evaluate(phrase : &Phrase, environment: &mut Environment) -> Result<Evaluation, EvaluationError> {
    match phrase {
        Phrase::None => Err(EvaluationError::new("None phrase")),
        Phrase::Primary(primitive) => evaluate_primitive(primitive, environment),
        Phrase::Postfix { noun, adjective } => todo!(),
        Phrase::Prefix { prefix, noun } => todo!(),
        Phrase::Action { subject, verb, object } => todo!(),
        Phrase::Condition { left, conjunction, right } => todo!(),
    }
}

fn evaluate_primitive(primitive: &Primitive, environment: &Environment) -> Result<Evaluation, EvaluationError> {
    match primitive {
        Primitive::Number(value) => Ok(Evaluation::Number(value.parse::<f32>().unwrap_or_default())),
        Primitive::Text(value) => Ok(Evaluation::Text(value.clone())),
        Primitive::True => Ok(Evaluation::Boolean(true)),
        Primitive::False => Ok(Evaluation::Boolean(false)),
        Primitive::It => todo!(),
        Primitive::Collective(phrases) => todo!(),
        Primitive::Variable(name) => if let Some(value) = environment.get(name) {
            Ok(value.clone())
        } else {
            Err(EvaluationError::new(&format!("Undefined variable \"{}\".", name)))
        },
    }
}