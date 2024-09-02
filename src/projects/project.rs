use std::fs;
use std::path::Path;
use serde::Serialize;

use crate::resourses::{RESOURCES_DIR, TEMPLATE_DIR};
use crate::projects::SANDBOX;

#[derive(Serialize)]
pub struct Project {
    pub name: String,
    pub version: String,
}

impl Project {
    const SOURCE_DIR: &'static str = "source";
    const ENTRY_FILE: &'static str = "main.prs";

    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version: "0.1.0".to_string(),
        }
    }

    pub fn create_entrypoint() -> std::io::Result<()> {
        let source_path = Path::new(SANDBOX).join(Project::SOURCE_DIR);
        if !source_path.exists() {
            fs::create_dir(source_path)?;
        }

        let template_filepath = Path::new(RESOURCES_DIR).join(TEMPLATE_DIR).join(Project::ENTRY_FILE);
        let entry_filepath = Path::new(SANDBOX).join(Project::SOURCE_DIR).join(Project::ENTRY_FILE);
        if !entry_filepath.exists() {
            fs::copy(template_filepath, entry_filepath)?;
        }
        
        Ok(())
    }
}
