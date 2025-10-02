use std::rc::Rc;

use crate::compilation::datatype::Datatype;
use crate::compilation::errors::EvaluationError;
use crate::compilation::evaluation::Evaluation;
use crate::compilation::intepreter::Intepreter;
use crate::compilation::statement::{Statement, Statements};

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Routine {
    pub name: Rc<str>,
    pub subject_type: Option<Datatype>,
    pub object_declarations: Rc<[Statement]>,
    pub instructions: Statements,
}

impl Routine {
    pub fn new(name: &str, subject_type: Option<Datatype>, object_declarations: Rc<[Statement]>, instructions: &Statements) -> Self {
        Self { 
            name: name.into(), 
            subject_type,
            object_declarations,
            instructions: instructions.clone(), 
        }
    }

    pub fn validate_subject(&self, subject: &Evaluation) -> Result<(), EvaluationError> {
        match (subject, self.subject_type.clone()) {
            (Evaluation::Void, None) => Ok(()),
            (subject, datatype) => {
                if subject.datatype() == datatype {
                    Ok(())
                } else {
                    let expected_type = datatype.map_or("Void".into(), |dt| format!("{dt}"));
                    let found_type = subject.datatype().map_or("Void".into(), |dt| format!("{dt}"));
                    Err(EvaluationError::new(&format!("Invalid subject type, expected {expected_type} but found {found_type}")))
                }
            },
        }
    }

    pub fn validate_object(&self, object: &Evaluation, intepreter: &mut Intepreter) -> Result<(), EvaluationError> {
        let parameters = self.object_declarations.iter()
            .map(|param_statement| intepreter.execute(param_statement))
            .collect::<Result<Vec<_>, _>>()?;

        let parameters = if parameters.is_empty() {
            Evaluation::Void
        } else if parameters.len() == 1 {
            parameters.first().unwrap().clone()
        } else {
            Evaluation::Collective(parameters.into())
        };
        
        if let Err(err_msg) = parameters.parity(&object) {
            Err(EvaluationError::new(&format!("Invalid object(s) for action {}, {}", self.name, err_msg)))
        } else {
            Ok(())
        }
    }
}
