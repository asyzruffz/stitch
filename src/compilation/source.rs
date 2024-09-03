use std::fs;
use std::path::Path;
use std::rc::Rc;

use crate::resourses::SANDBOX;
use crate::projects::project::Project;
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
        let source_directory = Path::new(SANDBOX).join(Project::SOURCE_DIR);
        let full_path = source_directory.join(self.path.as_ref());
        Ok(fs::read_to_string(full_path)?.as_str().into())
    }
}
