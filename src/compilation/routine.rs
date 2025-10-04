use std::fmt::Debug;
use std::rc::Rc;

use crate::compilation::datatype::Datatype;
use crate::compilation::errors::EvaluationError;
use crate::compilation::evaluation::Evaluation;
use crate::compilation::intepreter::Intepreter;
use crate::compilation::statement::Statements;
use crate::compilation::variable::VariableValue;


#[derive(Default, PartialEq, Clone, Debug)]
pub struct Routine {
    pub name: Rc<str>,
    pub subject_type: Option<Datatype>,
    pub object_parameters: Rc<[VariableValue]>,
    instruction: Instruction,
}

impl Routine {
    pub fn new(name: &str, subject_type: Option<Datatype>, object_parameters: Rc<[VariableValue]>, instructions: &Statements) -> Self {
        Self { 
            name: name.into(), 
            subject_type,
            object_parameters,
            instruction: Instruction::Custom(instructions.clone()),
        }
    }

    pub fn new_native(name: &str, subject_type: Option<Datatype>, object_parameters: Rc<[VariableValue]>, func: fn(Evaluation) -> Result<Evaluation, EvaluationError>) -> Self {
        Self { 
            name: name.into(), 
            subject_type,
            object_parameters,
            instruction: Instruction::BuiltIn(BuiltInInstruction(Box::new(func))),
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
                    Err(EvaluationError::new(&format!("Invalid subject type for action {}, expected {expected_type} but found {found_type}", self.name)))
                }
            },
        }
    }

    pub fn validate_object(&self, object: &Evaluation) -> Result<(), EvaluationError> {
        let parameters: Evaluation = self.object_parameters.clone().into();

        if let Err(err_msg) = parameters.parity(&object) {
            Err(EvaluationError::new(&format!("Invalid object(s) for action {}, {}", self.name, err_msg)))
        } else {
            Ok(())
        }
    }

    pub fn execute_using(&self, intepreter: &mut Intepreter) -> Result<Evaluation, EvaluationError> {
        match self.instruction {
            Instruction::NoOp => Ok(Evaluation::Void),

            Instruction::BuiltIn(ref func) => {
                let parameters: Evaluation = self.object_parameters.clone().into();
                func.0.as_ref()(parameters)
            },

            Instruction::Custom(ref statements) => {
                for statement in statements.0.as_ref() {
                    let eval = intepreter.execute(statement)?;
                    if let Evaluation::Conclusion(val) = eval {
                        return Ok(*val);
                    }
                }
                Ok(Evaluation::Void)
            },
        }
    }
}

impl Into<Evaluation> for Rc<[VariableValue]> {
    fn into(self) -> Evaluation {
        let parameters = self.iter()
            .map(|param| param.value.to_owned())
            .collect::<Vec<_>>();

        if parameters.is_empty() {
            Evaluation::Void
        } else if parameters.len() == 1 {
            parameters.first().unwrap().clone()
        } else {
            Evaluation::Collective(parameters.into())
        }
    }
}

#[derive(Default, PartialEq, Clone, Debug)]
enum Instruction {
    #[default] NoOp,
    BuiltIn(BuiltInInstruction),
    Custom(Statements),
}

struct BuiltInInstruction(Box<dyn Fn(Evaluation) -> Result<Evaluation, EvaluationError>>);

impl Default for BuiltInInstruction {
    fn default() -> Self {
        Self(Box::new(|_| Ok(Evaluation::Void)))
    }
}

impl PartialEq for BuiltInInstruction {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl Clone for BuiltInInstruction {
    fn clone(&self) -> Self {
        Self(Box::new(|_| Ok(Evaluation::Void)))
    }
}

impl Debug for BuiltInInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BuiltInInstruction")
    }
    
}
