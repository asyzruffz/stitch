use std::fmt;
use std::io;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum CompilerError {
    None,
    SourceError(Rc<str>),
    LexicalError(Rc<str>),
    RuntimeError(EvaluationError),
    MultiError(Rc<[CompilerError]>),
}

impl CompilerError {
    pub fn add(self, error: CompilerError) -> CompilerError {
        match self {
            CompilerError::None => error,
            CompilerError::MultiError(errs) => {
                let mut errors = errs.to_vec();
                errors.push(error);
                CompilerError::MultiError(errors.into())
            },
            other => CompilerError::MultiError(vec![other, error].into()),
        }
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompilerError::None => write!(f, "Non error"),
            CompilerError::SourceError(error) => write!(f, "Failed to read file: {}", error),
            CompilerError::LexicalError(error) => write!(f, "Parsed with error(s): {}", error),
            CompilerError::RuntimeError(error) => write!(f, "Evaluated with {} error(s): {}", error.error_count(), error),
            CompilerError::MultiError(errors) => write!(f, "Compiled with {} error(s):\n    {}", errors.len(), 
                errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n    ")),
        }
    }
}

impl From<io::Error> for CompilerError {
    fn from(error: io::Error) -> Self {
        CompilerError::SourceError(error.to_string().as_str().into())
    }
}

impl From<walkdir::Error> for CompilerError {
    fn from(error: walkdir::Error) -> Self {
        CompilerError::SourceError(error.to_string().as_str().into())
    }
}

impl From<Box<bincode::ErrorKind>> for CompilerError {
    fn from(error: Box<bincode::ErrorKind>) -> Self {
        CompilerError::SourceError(error.to_string().as_str().into())
    }
}

impl From<EvaluationError> for CompilerError {
    fn from(error: EvaluationError) -> Self {
        CompilerError::RuntimeError(error)
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct EvaluationError {
    details: Vec<Rc<str>>
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details.join(", "))
    }
}

impl EvaluationError {
    pub fn new(detail: &str) -> Self {
        Self {
            details: vec![detail.into()].into()
        }
    }

    pub fn add(mut self, detail: &str) -> Self {
        self.details.push(detail.into());
        Self {
            details: self.details
        }
    }

    pub fn concat(mut self, error: EvaluationError) -> Self {
        self.details.extend(error.details);
        Self {
            details: self.details
        }
    }

    pub fn concat_if(self, error: Option<EvaluationError>) -> Self {
        match error {
            Some(err) => self.concat(err),
            None => self
        }
    }

    pub fn error_count(&self) -> u32 {
        self.details.len().try_into().unwrap_or_default()
    }
}
