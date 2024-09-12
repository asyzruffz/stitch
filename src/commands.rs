use stitch::compilation::compiler::Compiler;
use stitch::projects::{config::Config, project::Project};

pub fn create_project(name: &str) {
    let result = Config::create(name)
        .and_then(|_| Project::create_entrypoint());
    
    if let Err(error) = result {
        eprintln!("{}", error);
    }
}

pub fn build_project() {
    let result = Compiler::new()
        .and_then(Compiler::tokenize)
        .and_then(Compiler::parse);

    if let Err(error) = result {
        eprintln!("{}", error);
    }
}

pub fn clean_project() {
    let result = Compiler::clean();

    if let Err(error) = result {
        eprintln!("{}", error);
    }
}

pub fn clean_and_build_project() {
    let result = Compiler::clean()
        .and_then(|_| Compiler::new())
        .and_then(Compiler::tokenize)
        .and_then(Compiler::parse);

    if let Err(error) = result {
        eprintln!("{}", error);
    }
}

pub fn run_project() {
    
}

pub fn test_project() {
    
}
