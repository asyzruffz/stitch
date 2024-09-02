use std::io::{self, Write};

use crate::projects::{config::Config, project::Project};

pub fn create_project(name: &str) {
    let result = Config::create(name)
        .and_then(|_| Project::create_entrypoint());
    
    if let Err(error) = result {
        writeln!(io::stderr(), "{}", error).unwrap();
    }
}

pub fn build_project() {
    
}

pub fn run_project() {
    
}

pub fn test_project() {
    
}
