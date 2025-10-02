use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::compilation::conjunction::Conjunction;
use crate::compilation::datatype::{Datatype, VerbType};
use crate::compilation::environment::{Environment, Variable};
use crate::compilation::errors::EvaluationError;
use crate::compilation::evaluation::Evaluation;
use crate::compilation::phrase::Phrase;
use crate::compilation::prefix::Prefix;
use crate::compilation::primitive::Primitive;
use crate::compilation::routine::Routine;
use crate::compilation::statement::{Statement, Statements };
use crate::compilation::substantive::Substantive;
use crate::compilation::verb::Verb;


#[derive(Default)]
pub struct Intepreter {
    environment: Rc<RefCell<Environment>>,
}

impl Intepreter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn execute(&mut self, statement : &Statement) -> Result<Evaluation, EvaluationError> {
        match statement {
            Statement::Noun { name, super_type, body } => declare_noun(name, super_type.as_ref(), body, self.environment.clone()),
            Statement::Verb { name, hence_type, subject_type, object_declarations, body } => 
                declare_verb(name, hence_type.as_ref(), subject_type.clone(), object_declarations.clone(), body, self.environment.clone()),
            Statement::Adjective { name, subject_type, body } => declare_adjective(name, subject_type, body, self.environment.clone()),
            Statement::So { name, datatype, initializer } => declare_so(name, datatype, initializer.as_ref(), self.environment.clone()),
            Statement::Phrase(phrase) => evaluate(phrase, self.environment.clone()),
            Statement::Hence(phrase) => evaluate_hence(phrase, self.environment.clone()),
        }
    }
    
    pub fn within_scope(outer: Rc<RefCell<Environment>>) -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::within_scope(outer))),
        }
    }

    pub fn define(&mut self, var: Variable, value: Evaluation) {
        self.environment.borrow_mut().define(var, value);
    }

    pub fn define_subject(&mut self, subject: Evaluation) {
        if subject == Evaluation::Void {
            return;
        }
        
        match subject.datatype() {
            Some(datatype) => self.define(Variable::new("it", &datatype), subject),
            None => self.define(Variable::with("it"), subject),
        };
    }

    pub fn define_object(&mut self, object: Evaluation, parameters: &[Statement]) -> Result<(), EvaluationError> {
        let arguments = match object {
            Evaluation::Void => return Err(EvaluationError::new("Invalid object for definition.")),
            Evaluation::Collective(objs) => parameters.iter().zip(objs.as_ref().into_iter().cloned().collect::<Vec<_>>()).collect::<Vec<_>>(),
            obj => parameters.iter().zip(std::iter::once(obj)).collect::<Vec<_>>(),
        };

        arguments.into_iter()
            .map(|(param, obj)| match param {
                Statement::So { name, datatype, initializer } => {
                    declare_so(name, datatype, initializer.as_ref(), self.environment.clone())?;
                    let variable = Variable::new(name, datatype);
                    match self.environment.borrow_mut().assign(variable, obj) {
                        Err(variable) => Err(EvaluationError::new(&format!("Undefined variable {}.", variable))),
                        Ok(_) => Ok(()),
                    }
                },
                _ => Err(EvaluationError::new("Invalid param statement")),
            })
            .collect()
    }
}

fn declare_noun(name: &str, super_type: Option<&Datatype>, body: &Statements, environment: Rc<RefCell<Environment>>) -> Result<Evaluation, EvaluationError> {
    let noun_environment = Rc::new(RefCell::new(Environment::within_scope(environment.clone())));
    
    if let Some(super_type) = super_type {
        match super_type {
            Datatype::Noun(super_type_name) => noun_environment.borrow_mut()
                .define(Variable::new("super", super_type), Evaluation::Noun(Substantive::new(super_type_name, noun_environment.clone()))),
            Datatype::Number => noun_environment.borrow_mut().define(Variable::new("super", super_type), Evaluation::Number(0.0)),
            Datatype::Text => noun_environment.borrow_mut().define(Variable::new("super", super_type), Evaluation::Text("".into())),
            Datatype::Boolean => noun_environment.borrow_mut().define(Variable::new("super", super_type), Evaluation::Boolean(false)),
            Datatype::Verb(super_type_name) => return Err(EvaluationError::new(&format!("Invalid verb {super_type_name} as super type for noun."))),
            Datatype::Adjective(super_type_name) => return Err(EvaluationError::new(&format!("Invalid adjective {super_type_name} as super type for noun."))),
        }
    }

    for statement in body.0.as_ref() {
        match statement {
            Statement::Noun { name, super_type, body } => declare_noun(name, super_type.as_ref(), body, noun_environment.clone())?,
            Statement::Verb { name, hence_type, subject_type, object_declarations, body } => 
                declare_verb(name, hence_type.as_ref(), subject_type.clone(), object_declarations.clone(), body, noun_environment.clone())?,
            Statement::Adjective { name, subject_type, body } => declare_adjective(name, subject_type, body, noun_environment.clone())?,
            Statement::So { name, datatype, initializer } => declare_so(name, datatype, initializer.as_ref(), noun_environment.clone())?,
            _ => return Err(EvaluationError::new("Invalid statement in noun body.")),
        };
    }

    let variable = Variable::new(name, &Datatype::Noun(name.into()));
    environment.borrow_mut().define(variable, Evaluation::Noun(Substantive::new(name, noun_environment)));
    Ok(Evaluation::Void)
}

fn declare_verb(name: &str, hence_type: Option<&Datatype>, subject_type: Option<Datatype>, object_declarations: Rc<[Statement]>, body: &Statements, environment: Rc<RefCell<Environment>>) -> Result<Evaluation, EvaluationError> {
    let variable = match hence_type {
        Some(datatype) => Variable::new(name, &Datatype::Verb(VerbType::new(name, Some(datatype.clone())))),
        None => Variable::new(name, &Datatype::Verb(VerbType::new(name, None))),
    };

    environment.borrow_mut().define(variable, Evaluation::Action(Routine::new(name, subject_type, object_declarations, body)));
    Ok(Evaluation::Void)
}

fn declare_adjective(name: &str, subject_type: &Datatype, body: &Statements, environment: Rc<RefCell<Environment>>) -> Result<Evaluation, EvaluationError> {
    let variable = Variable::new(name, &Datatype::Adjective(name.into()));
    environment.borrow_mut().define(variable, Evaluation::Adjective(Routine::new(name, Some(subject_type.clone()), Rc::default(), body)));
    Ok(Evaluation::Void)
}

fn declare_so(name: &str, datatype: &Datatype, initializer : Option<&Phrase>, environment: Rc<RefCell<Environment>>) -> Result<Evaluation, EvaluationError> {
    let variable = Variable::new(name, datatype);
    match initializer {
        None => {
            let default_value = match datatype {
                Datatype::Number => Evaluation::Number(0.0),
                Datatype::Text => Evaluation::Text("".into()),
                Datatype::Boolean => Evaluation::Boolean(false),
                Datatype::Noun(name) => Evaluation::Void, //TODO: Implement noun default constructor
                Datatype::Verb(verb) => Evaluation::Void, //TODO: Implement anonymous default verb
                Datatype::Adjective(name) => Evaluation::Void, //TODO: Implement anonymous default adjective
            };
            environment.borrow_mut().define(variable, default_value.clone());
            Ok(default_value)
        },
        Some(phrase) => {
            let result = evaluate(phrase, environment.clone())?;
            match result {
                Evaluation::Void => Err(EvaluationError::new("Unable to initialize so declaration with Void phrase.")),
                value => {
                    environment.borrow_mut().define(variable, value.clone());
                    Ok(value)
                },
            }
        },
    }
}

fn evaluate(phrase : &Phrase, environment: Rc<RefCell<Environment>>) -> Result<Evaluation, EvaluationError> {
    match phrase {
        Phrase::None => Err(EvaluationError::new("None phrase")),
        Phrase::Primary(primitive) => evaluate_primitive(primitive, environment.clone()),
        Phrase::Postfix { noun, adjective } => evaluate_postfix(noun, adjective, environment.clone()),
        Phrase::Prefix { prefix, noun } => evaluate_prefix(prefix, noun, environment.clone()),
        Phrase::Action { subject, verb, object } => evaluate_action(verb, subject.as_deref(), object.as_deref(), environment.clone()),
        Phrase::Condition { left, conjunction, right } => evaluate_condition(conjunction, left, right, environment),
    }
}

fn evaluate_hence(phrase : &Phrase, environment: Rc<RefCell<Environment>>) -> Result<Evaluation, EvaluationError> {
    let hence_value = evaluate(phrase, environment.clone())?;
    match hence_value {
        Evaluation::Void => Err(EvaluationError::new("Invalid void as hence value")),
        skip @ Evaluation::Skip(_) => Ok(Evaluation::Conclusion(Box::new(skip))),
        value => {
            if let Evaluation::Boolean(condition) = evaluate_truth(value.clone(), environment.clone())? {
                if condition {
                    Ok(Evaluation::Conclusion(Box::new(value)))
                } else {
                    Ok(Evaluation::Conclusion(Box::new(Evaluation::Skip(Box::new(value)))))
                }
            } else {
                Ok(Evaluation::Conclusion(Box::new(value)))
            }
        },
    }
}

fn evaluate_primitive(primitive: &Primitive, environment: Rc<RefCell<Environment>>) -> Result<Evaluation, EvaluationError> {
    match primitive {
        Primitive::Number(value) => Ok(Evaluation::Number(value.parse::<f32>().unwrap_or_default())),
        Primitive::Text(value) => Ok(Evaluation::Text(value.clone())),
        Primitive::True => Ok(Evaluation::Boolean(true)),
        Primitive::False => Ok(Evaluation::Boolean(false)),
        Primitive::It => match environment.borrow().get("it") {
            Some(value) => Ok(value),
            None => Err(EvaluationError::new("Undefined variable \"it\".")),
        },
        Primitive::Collective(phrases) => phrases.iter()
            .map(|phrase| evaluate(phrase, environment.clone()))
            .collect::<Result<Vec<_>, _>>()
            .map(|evaluations| Evaluation::Collective(evaluations.into())),
        Primitive::Variable(name) => match environment.borrow().get(name) {
            Some(value) => Ok(value),
            None => Err(EvaluationError::new(&format!("Undefined variable \"{}\".", name))),
        },
    }
}

fn evaluate_postfix(subject_phrs: &Phrase, adjective: &Phrase, environment: Rc<RefCell<Environment>>) -> Result<Evaluation, EvaluationError> {
    let subject = evaluate(subject_phrs, environment.clone())?;
    let adjective = evaluate(adjective, environment.clone())?;
    evaluate_qualifier(subject, adjective, environment.clone())
}

fn evaluate_prefix(prefix: &Prefix, subject_phrs: &Phrase, environment: Rc<RefCell<Environment>>) -> Result<Evaluation, EvaluationError> {
    match prefix {
        Prefix::Not => match evaluate(subject_phrs, environment.clone())? {
            Evaluation::Void => Err(EvaluationError::new("Invalid not prefix for void")),
            Evaluation::Skip(noun) => Ok(noun.deref().to_owned()),
            Evaluation::Conclusion(_) => Err(EvaluationError::new("Invalid not prefix for conclusion")),
            Evaluation::Number(_) => Err(EvaluationError::new("Invalid not prefix for number")),
            Evaluation::Text(_) => Err(EvaluationError::new("Invalid not prefix for text")),
            Evaluation::Boolean(value) => Ok(Evaluation::Boolean(!value)),
            Evaluation::Collective(_) => Err(EvaluationError::new("Invalid not prefix for collective")),
            noun @ Evaluation::Noun(_) => Ok(Evaluation::Skip(Box::new(noun))),
            Evaluation::Action(routine) => Err(EvaluationError::new(&format!("Invalid not prefix for action {}", routine.name))),
            Evaluation::Adjective(routine) => Err(EvaluationError::new(&format!("No implementation of not prefix for adjective {}", routine.name))),
        },
        Prefix::Negation => match evaluate(subject_phrs, environment.clone())? {
            Evaluation::Void => Err(EvaluationError::new("Invalid negation prefix for void")),
            Evaluation::Skip(_) => Err(EvaluationError::new("Invalid negation prefix for skip")),
            Evaluation::Conclusion(_) => Err(EvaluationError::new("Invalid negation prefix for conclusion")),
            Evaluation::Number(value) => Ok(Evaluation::Number(-value)),
            Evaluation::Text(_) => Err(EvaluationError::new("Invalid negation prefix for text")),
            Evaluation::Boolean(_) => Err(EvaluationError::new("Invalid negation prefix for boolean")),
            Evaluation::Collective(_) => Err(EvaluationError::new("Invalid negation prefix for collective")),
            Evaluation::Noun(substantive) => Err(EvaluationError::new(&format!("Invalid negation prefix for {}.", substantive.name))),
            Evaluation::Action(routine) => Err(EvaluationError::new(&format!("Invalid negation prefix for action {}.", routine.name))),
            Evaluation::Adjective(routine) => Err(EvaluationError::new(&format!("Invalid negation prefix for adjective {}.", routine.name))),
        },
        Prefix::Adjective(name) => match environment.borrow().get(name) {
            Some(adjective) => {
                let subject = evaluate(subject_phrs, environment.clone())?;
                evaluate_qualifier(subject, adjective, environment.clone())
            },
            None => Err(EvaluationError::new(&format!("Undefined adjective {}.", name))),
        },
        Prefix::None => Err(EvaluationError::new("None prefix invalid")),
    }
}

fn evaluate_action(verb: &Verb, subject_phrs : Option<&Phrase>, object_phrs : Option<&Phrase>, environment: Rc<RefCell<Environment>>) -> Result<Evaluation, EvaluationError> {
    let subject = match subject_phrs {
        Some(subject_phrs) => evaluate(subject_phrs, environment.clone())?,
        None => Evaluation::Void,
    };
    let object = match object_phrs {
        Some(object_phrs) => evaluate(object_phrs, environment.clone())?,
        None => Evaluation::Void,
    };

    match verb {
        Verb::Divide | Verb::Multiply | Verb::Subtract | Verb::Add => match (subject, object) {
            (Evaluation::Number(lvalue), Evaluation::Number(rvalue)) => {
                match verb {
                    Verb::Divide => Ok(Evaluation::Number(lvalue / rvalue)),
                    Verb::Multiply => Ok(Evaluation::Number(lvalue * rvalue)),
                    Verb::Subtract => Ok(Evaluation::Number(lvalue - rvalue)),
                    Verb::Add => Ok(Evaluation::Number(lvalue + rvalue)),
                    _ => unreachable!(),
                }
            }
            _ => Err(EvaluationError::new("Invalid operand not as numbers")),
        },
        Verb::Assign => {
            if let skipped_subject @ Evaluation::Skip(_) = subject {
                return Ok(skipped_subject);
            }
            if let Evaluation::Skip(_) = object {
                return Ok(subject);
            }

            match subject_phrs {
                Some(subject_phrs) => match subject_phrs {
                    Phrase::Primary(Primitive::Variable(name)) => {
                        match environment.borrow_mut().assign(Variable::with(name), object.clone()) {
                            Err(variable) => Err(EvaluationError::new(&format!("Undefined variable {}.", variable))),
                            Ok(_) => Ok(object),
                        }
                    }
                    _ => Err(EvaluationError::new("Invalid assignment target")),
                },
                None => Err(EvaluationError::new("Invalid assigning to none")),
            }
        },
        Verb::Action(name) => {
            if let skipped_subject @ Evaluation::Skip(_) = subject {
                return Ok(skipped_subject);
            }
            if let Evaluation::Skip(_) = object {
                return Ok(subject);
            }

            if let Some(action) = environment.borrow().get(name.as_ref()) {
                match action {
                    Evaluation::Action(routine) => evaluate_routine(subject, object, &routine, environment.clone()),
                    _ => Err(EvaluationError::new(&format!("Invalid action {}.", name.as_ref()))),
                }
            } else {
                Err(EvaluationError::new(&format!("Undefined verb {}.", name.as_ref())))
            }
        },
        Verb::None => Err(EvaluationError::new("None verb invalid")),
    }
}

fn evaluate_condition(conjunction: &Conjunction, left : &Phrase, right : &Phrase, environment: Rc<RefCell<Environment>>) -> Result<Evaluation, EvaluationError> {
    match conjunction {
        Conjunction::Greater | Conjunction::GreaterEqual | Conjunction::Less | Conjunction::LessEqual => {
            let left_phrs = evaluate(left, environment.clone());
            let right_phrs = evaluate(right, environment.clone());

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
            let left_phrs = evaluate(left, environment.clone());
            let right_phrs = evaluate(right, environment.clone());

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
            let result = evaluate_truth(evaluate(left, environment.clone())?, environment.clone())?;
            match result {
                Evaluation::Boolean(value) => {
                    if !value { Ok(result) }
                    else {
                        let result = evaluate_truth(evaluate(right, environment.clone())?, environment.clone())?;
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
            let result = evaluate_truth(evaluate(left, environment.clone())?, environment.clone())?;
            match result {
                Evaluation::Boolean(value) => {
                    if value { Ok(result) }
                    else {
                        let result = evaluate_truth(evaluate(right, environment.clone())?, environment.clone())?;
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

fn evaluate_qualifier(subject: Evaluation, adjective: Evaluation, environment: Rc<RefCell<Environment>>) -> Result<Evaluation, EvaluationError> {
    match adjective {
        Evaluation::Boolean(value) => if value {
            Ok(subject)
        } else {
            Ok(Evaluation::Skip(Box::new(subject)))
        },
        Evaluation::Adjective(routine) => {
            evaluate_routine(subject, Evaluation::Void, &routine, environment.clone())
        },
        Evaluation::Skip(_) => Ok(Evaluation::Skip(Box::new(subject))),
        Evaluation::Void => Err(EvaluationError::new("Invalid void as adjective")),
        Evaluation::Conclusion(_) => Err(EvaluationError::new("Invalid conclusion as adjective")),
        Evaluation::Number(_) => Err(EvaluationError::new("Invalid number as adjective")),
        Evaluation::Text(_) => Err(EvaluationError::new("Invalid text as adjective")),
        Evaluation::Collective(_) => Err(EvaluationError::new("Invalid collective as adjective")),
        Evaluation::Noun(substantive) => Err(EvaluationError::new(&format!("Invalid noun {} as adjective", substantive.name))),
        Evaluation::Action(routine) => Err(EvaluationError::new(&format!("Invalid action {} as adjective", routine.name))),
    }
}

fn evaluate_routine(subject: Evaluation, object: Evaluation, routine: &Routine, environment: Rc<RefCell<Environment>>) -> Result<Evaluation, EvaluationError> {
    let mut intepreter = Intepreter::within_scope(environment);
    
    routine.validate_subject(&subject)?;
    intepreter.define_subject(subject.clone());

    routine.validate_object(&object, &mut intepreter)?;
    match object {
        Evaluation::Void | Evaluation::Skip(_) => {},
        obj => intepreter.define_object(obj, routine.object_declarations.as_ref())?,
    };

    for statement in routine.instructions.0.as_ref() {
        let eval = intepreter.execute(statement)?;
        if let Evaluation::Conclusion(val) = eval {
            return Ok(*val);
        }
    }
    Ok(Evaluation::Void)
}

fn evaluate_truth(value : Evaluation, environment: Rc<RefCell<Environment>>) -> Result<Evaluation, EvaluationError> {
    match value {
        Evaluation::Void => Err(EvaluationError::new("Invalid boolean condition for void")),
        Evaluation::Skip(_) => Err(EvaluationError::new("Invalid boolean condition for skip")),
        Evaluation::Conclusion(_) => Err(EvaluationError::new("Invalid boolean condition for conclusion")),
        Evaluation::Number(value) => Ok(Evaluation::Boolean(value != 0.0)),
        Evaluation::Text(_) => Ok(Evaluation::Boolean(true)),
        Evaluation::Boolean(value) => Ok(Evaluation::Boolean(value)),
        Evaluation::Collective(evaluations) => Ok(Evaluation::Boolean(evaluations.iter()
            .map(|evaluation| match evaluate_truth(evaluation.clone(), environment.clone()) {
                Ok(Evaluation::Boolean(value)) => Ok(value),
                Ok(_) => Err(EvaluationError::new("Invalid boolean condition for collective element")),
                Err(error) => Err(error),
            })
            .collect::<Result<Vec<_>, _>>()?.into_iter()
            .all(|value| value))),
        Evaluation::Noun(substantive) => Err(EvaluationError::new(&format!("No implementation of truth for noun {}", substantive.name))),
        Evaluation::Action(_) => Err(EvaluationError::new("Invalid boolean condition for action")),
        Evaluation::Adjective(routine) => Err(EvaluationError::new(&format!("No implementation of truth for adjective {}", routine.name))),
    }
}
