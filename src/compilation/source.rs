use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use crate::projects::project::Project;
use crate::compilation::errors::CompilerError;

#[derive(Debug, Default)]
pub struct Source {
    pub path: Rc<str>,
    pub filename: Rc<str>,
    pub hash: Rc<[u8]>,
}

impl Source {
    pub fn new(path: &str, filename: &str, hash: &[u8]) -> Result<Self, CompilerError> {
        Ok(Self {
            path: path.into(),
            filename: filename.into(),
            hash: hash.into(),
        })
    }

    pub fn content(&self) -> Result<Rc<str>, CompilerError> {
        Ok(fs::read_to_string(self.full_path()?)?.as_str().into())
    }

    pub fn full_path(&self) -> Result<PathBuf, CompilerError> {
        let source_directory = Project::get_source_dir(false)?;
        let full_path = source_directory.join(self.path.as_ref());
        Ok(full_path)
    }
}
