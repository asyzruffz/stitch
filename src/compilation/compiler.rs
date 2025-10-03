use std::cell::RefCell;
use std::rc::Rc;
use std::fs;

use walkdir::WalkDir;

use crate::compilation::environment::Environment;
use crate::compilation::errors::CompilerError;
use crate::compilation::intepreter::Intepreter;
use crate::compilation::intermediate::Intermediate;
use crate::compilation::parser::Parser;
use crate::compilation::source::Source;
use crate::compilation::scanner::{self, Scanner};
use crate::compilation::statement::Statement;
use crate::compilation::std::add_builtin_features;
use crate::compilation::token::Token;
use crate::projects::project::Project;
use crate::utils::hasher::hash_file;

pub trait CompilerState {}

pub struct Compiler<State: CompilerState = Initial> {
    state: State,
}

#[derive(Default)]
pub struct Initial;
#[derive(Default)]
pub struct Ready {
    pub sources : Rc<[Source]>,
}
#[derive(Default)]
pub struct Tokenized {
    pub tokens : Rc<[Token]>,
}
#[derive(Default)]
pub struct Parsed {
    pub statements : Rc<[Statement]>,
}
#[derive(Default)]
pub struct Evaluated;

impl CompilerState for Initial {}
impl CompilerState for Ready {}
impl CompilerState for Tokenized {}
impl CompilerState for Parsed {}
impl CompilerState for Evaluated {}

impl Compiler<Initial> {
    pub fn new() -> Result<Compiler<Ready>, CompilerError> {
        let source_directory = Project::get_source_dir(false)?;

        let mut sources = Vec::new();
        for entry in WalkDir::new(source_directory.as_path()) {
            let entry = entry?;
            if entry.file_type().is_dir() {
                continue;
            }
            
            let full_path = entry.path();
            let path = full_path.strip_prefix(source_directory.as_path())
                .map_err(|e| CompilerError::SourceError(e.to_string().as_str().into()))?;
            let path = path.to_string_lossy();

            let filename = entry.file_name().to_string_lossy();

            if filename.ends_with(".prs") {
                let hash = hash_file(full_path)?;
                let source = Source::new(path.as_ref(), filename.as_ref(), hash.as_slice())?;
                sources.push(source);
            }
        }
        
        Ok(Compiler {
            state: Ready { sources: sources.into() }
        })
    }

    pub fn clean() -> Result<Compiler<Initial>, CompilerError> {
        if let Ok(intermediate_path) = Project::get_intermediate_dir(false) {
            fs::remove_dir_all(intermediate_path)?;
        }

        Ok(Compiler { state: Initial })
    }
}

impl Compiler<Ready> {
    pub fn tokenize(self) -> Result<Compiler<Tokenized>, CompilerError> {
        let scanners = self.state.sources.iter()
            .map(to_token)
            .collect::<Result<Vec<_>, CompilerError>>()?;

        let tokens = scanners.into_iter()
            .map(|scanner| scanner.intermediate().tokens.to_vec())
            .flatten()
            .collect::<Vec<_>>();

        /*for token in &tokens {
            println!("{token}");
        }*/

        Ok(Compiler {
            state: Tokenized { tokens: tokens.into() }
        })
    }
}

impl Compiler<Tokenized> {
    pub fn parse(self) -> Result<Compiler<Parsed>, CompilerError> {
        let parser = Parser::new(self.state.tokens)
            .parse()?;
        
        /*for statement in parser.statements().as_ref() {
            println!("{statement}");
        }*/
    
        Ok(Compiler {
            state: Parsed { statements: parser.statements() }
        })
    }
}

impl Compiler<Parsed> {
    pub fn evaluate(self) -> Result<Compiler<Evaluated>, CompilerError> {
        let std_environment = Rc::new(RefCell::new(Environment::default()));
        add_builtin_features(std_environment.clone());

        let mut intepreter = Intepreter::within_scope(std_environment);

        for statement in self.state.statements.as_ref() {
            intepreter.execute(&statement)
                .map_err(|error| CompilerError::RuntimeError(error))?;
        }
        
        Ok(Compiler {
            state: Evaluated
        })
    }
}

fn to_token(source: &Source) -> Result<Scanner<scanner::Done>, CompilerError> {
    let result = match Intermediate::try_from(source) {
        Ok(intermediate) => Scanner::from(intermediate),
        Err(_) => {
            let scanner = Scanner::new(source.content()?.as_ref(), source.hash.clone())
                .tokenize();
            scanner
                .intermediate()
                .save_for(source)?;
            scanner
        },
    };

    Ok(result)
}
