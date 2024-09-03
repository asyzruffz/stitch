use std::fs::File;
use std::path::Path;
use std::io::Write;
use serde::Serialize;

use crate::resourses::SANDBOX;
use crate::compilation::errors::CompilerError;
use crate::projects::project::Project;

#[derive(Serialize)]
pub struct Config {
    pub project: Project,
}

impl Config {
    const FILENAME: &'static str = "Book.toml";

    pub fn create(project_name: &str) -> Result<(), CompilerError> {
        let config = Self {
            project: Project::new(project_name),
        };

        let toml = toml::to_string(&config).unwrap();

        let mut file = File::create(Path::new(SANDBOX).join(Config::FILENAME))?;
        write!(file, "{}", toml)?;
        Ok(())
    }
}
