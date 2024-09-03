use std::fs;

use crate::compilation::errors::CompilerError;

pub struct Source {
    path: String,
    filename: String,
}

impl Source {
    pub fn new<S: Into<String>>(path: S, filename: S) -> Result<Self, CompilerError> {
        Ok(Self {
            path: path.into(),
            filename: filename.into(),
        })
    }

    pub fn content(&self) -> Result<String, CompilerError> {
        Ok(fs::read_to_string(self.path.clone())?)
    }
}
