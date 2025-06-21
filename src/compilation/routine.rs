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
                    Err(EvaluationError::new("Invalid subject type"))
                }
            },
            _ => Err(EvaluationError::new("Invalid subject type")),
        }
    }

    pub fn validate_object(&self, object: &Evaluation, intepreter: &mut Intepreter) -> Result<(), EvaluationError> {
        let parameters = self.object_declarations.iter()
            .map(|param_statement| intepreter.execute(param_statement))
            .collect::<Result<Vec<_>, _>>()?;

        let parameters = Evaluation::Collective(parameters.into());
        if !parameters.parity(&object) {
            Err(EvaluationError::new("Invalid objects for action"))
        } else {
            Ok(())
        }
    }
}
