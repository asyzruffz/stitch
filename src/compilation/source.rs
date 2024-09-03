use std::fs;
use std::rc::Rc;

use crate::compilation::errors::CompilerError;

pub struct Source {
    path: Rc<str>,
    filename: Rc<str>,
}

impl Source {
    pub fn new(path: &str, filename: &str) -> Result<Self, CompilerError> {
        Ok(Self {
            path: path.into(),
            filename: filename.into(),
        })
    }

    pub fn content(&self) -> Result<Rc<str>, CompilerError> {
        Ok(fs::read_to_string(self.path.as_ref())?.as_str().into())
    }
}
