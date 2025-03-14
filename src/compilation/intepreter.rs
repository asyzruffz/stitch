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
        Phrase::Condition { left, conjunction, right } => evaluate_condition(conjunction, left, right, environment),
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

fn evaluate_condition(conjunction: &Conjunction, left : &Phrase, right : &Phrase, environment: &mut Environment) -> Result<Evaluation, EvaluationError> {
    match conjunction {
        Conjunction::Greater | Conjunction::GreaterEqual | Conjunction::Less | Conjunction::LessEqual => {
            let left_phrs = evaluate(left, environment);
            let right_phrs = evaluate(right, environment);

            let error = if let Err(err) = left_phrs.clone() { Some(err) } else { None };
            let error = if let Err(err) = right_phrs.clone() { error.and_then(|e| Some(e.concat(err))) } else { error };

            if let (Ok(Evaluation::Number(lvalue)),
                    Ok(Evaluation::Number(rvalue)))
                        = (left_phrs, right_phrs) {

                match conjunction {
                    Conjunction::Greater => Ok(Evaluation::Boolean(lvalue > rvalue)),
                    Conjunction::GreaterEqual => Ok(Evaluation::Boolean(lvalue >= rvalue)),
                    Conjunction::Less => Ok(Evaluation::Boolean(lvalue < rvalue)),
                    Conjunction::LessEqual => Ok(Evaluation::Boolean(lvalue <= rvalue)),
                    _ => unreachable!(),
                }
                
            } else {
                Err(EvaluationError::new("Invalid comparison operand not as numbers").concat_if(error))
            }
        },
        Conjunction::Equal | Conjunction::NotEqual => {
            let left_phrs = evaluate(left, environment);
            let right_phrs = evaluate(right, environment);

            let error = if let Err(err) = left_phrs.clone() { Some(err) } else { None };
            let error = if let Err(err) = right_phrs.clone() { error.and_then(|e| Some(e.concat(err))) } else { error };

            if let (Ok(left_result), Ok(right_result)) = (left_phrs, right_phrs) {

                match conjunction {
                    Conjunction::Equal => Ok(Evaluation::Boolean(Evaluation::equal(&left_result, &right_result))),
                    Conjunction::NotEqual => Ok(Evaluation::Boolean(!Evaluation::equal(&left_result, &right_result))),
                    _ => unreachable!(),
                }
                
            } else {
                Err(EvaluationError::new("Invalid equality operand").concat_if(error))
            }
        },
        Conjunction::And => {
            let result = evaluate_truth(evaluate(left, environment)?, environment)?;
            match result {
                Evaluation::Boolean(value) => {
                    if !value { Ok(result) }
                    else {
                        let result = evaluate_truth(evaluate(right, environment)?, environment)?;
                        match result {
                            Evaluation::Boolean(_) => Ok(result),
                            _ => Err(EvaluationError::new("Invalid logic operand")),
                        }
                    }
                }
                _ => Err(EvaluationError::new("Invalid logic operand")),
            }
        },
        Conjunction::Or => {
            let result = evaluate_truth(evaluate(left, environment)?, environment)?;
            match result {
                Evaluation::Boolean(value) => {
                    if value { Ok(result) }
                    else {
                        let result = evaluate_truth(evaluate(right, environment)?, environment)?;
                        match result {
                            Evaluation::Boolean(_) => Ok(result),
                            _ => Err(EvaluationError::new("Invalid logic operand")),
                        }
                    }
                }
                _ => Err(EvaluationError::new("Invalid logic operand")),
            }
        },
        Conjunction::None => Err(EvaluationError::new("None conjunction invalid")),
    }
}

fn evaluate_truth(value : Evaluation, environment: &mut Environment) -> Result<Evaluation, EvaluationError> {
    match value {
        Evaluation::Void => Err(EvaluationError::new("Invalid boolean condition for void")),
        Evaluation::Number(_) => Ok(Evaluation::Boolean(true)),
        Evaluation::Text(_) => Ok(Evaluation::Boolean(true)),
        Evaluation::Boolean(value) => Ok(Evaluation::Boolean(value)),
        Evaluation::Custom(var) => if let Some(value) = environment.get(var.as_ref()) {
            evaluate_truth(value.clone(), environment)
        } else {
            Err(EvaluationError::new(&format!("Undefined variable {}.", var.as_ref())))
        },
    }
}
