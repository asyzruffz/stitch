use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use serde::Serialize;

use crate::resourses::{RESOURCES_DIR, TEMPLATE_DIR, SANDBOX};
use crate::compilation::errors::CompilerError;

#[derive(Serialize)]
pub struct Project {
    pub name: Rc<str>,
    pub version: Rc<str>,
}

impl Project {
    pub const SOURCE_DIR: &'static str = "source";
    pub const INTERMEDIATE_DIR: &'static str = "intermediate";
    const ENTRY_FILE: &'static str = "main.prs";

    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            version: "0.1.0".into(),
        }
    }

    pub fn create_entrypoint() -> Result<(), CompilerError> {
        let source_path = Self::get_source_dir(true)?;

        let template_filepath = Path::new(RESOURCES_DIR).join(TEMPLATE_DIR).join(Project::ENTRY_FILE);
        let entry_filepath = source_path.join(Project::ENTRY_FILE);
        if !entry_filepath.exists() {
            fs::copy(template_filepath, entry_filepath)?;
        }
        
        Ok(())
    }

    pub fn get_source_dir(create_if_not_exist: bool) -> Result<PathBuf, CompilerError> {
        let source_path = Path::new(SANDBOX).join(Project::SOURCE_DIR);
        if !source_path.exists() {
            if create_if_not_exist {
                fs::create_dir(&source_path)?;
            } else {
                return Err(CompilerError::SourceError(Rc::from("Source directory not exist")));
            }
        }
    
        Ok(source_path)
    }

    pub fn get_intermediate_dir(create_if_not_exist: bool) -> Result<PathBuf, CompilerError> {
        let intermediate_path = Path::new(SANDBOX).join(Project::INTERMEDIATE_DIR);
        if !intermediate_path.exists() {
            if create_if_not_exist {
                fs::create_dir(&intermediate_path)?;
            } else {
                return Err(CompilerError::SourceError(Rc::from("Intermediate directory not exist")));
            }
        }
    
        Ok(intermediate_path)
    }
}
