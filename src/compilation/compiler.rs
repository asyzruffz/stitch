use std::path::Path;
use std::rc::Rc;

use walkdir::WalkDir;

use crate::utils::MapOkTrait;
use crate::resourses::SANDBOX;
use crate::projects::project::Project;
use crate::compilation::source::Source;
use crate::compilation::scanner::Scanner;
use crate::compilation::token::Token;
use crate::compilation::errors::CompilerError;

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
    //pub statements : Rc<[Statement]>,
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
        let source_directory = Path::new(SANDBOX).join(Project::SOURCE_DIR);

        let mut sources = Vec::new();
        for entry in WalkDir::new(&source_directory) {
            let entry = entry?;

            let path = entry.path();
            let path = path.strip_prefix(&source_directory)
                .map_err(|e| CompilerError::SourceError(e.to_string().as_str().into()))?;
            let path = path.to_string_lossy();

            let filename = entry.file_name().to_string_lossy();

            if filename.ends_with(".prs") {
                let source = Source::new(path.as_ref(), filename.as_ref())?;
                sources.push(source);
            }
        }
        
        Ok(Compiler {
            state: Ready { sources: sources.into() }
        })
    }
}

impl Compiler<Ready> {
    pub fn tokenize(self) -> Result<Compiler<Tokenized>, CompilerError> {
        let scanners = self.state.sources.iter()
            .map(|source| source.content())
            .map_ok(|source| Scanner::new(source.as_ref()).tokenize())
            .collect::<Result<Vec<_>, CompilerError>>()?;

        let tokens = scanners.into_iter()
            .map(|scanner| scanner.tokens().to_vec())
            .flatten()
            .collect::<Vec<_>>();

        Ok(Compiler {
            state: Tokenized { tokens: tokens.into() }
        })
    }
}

impl Compiler<Tokenized> {
    pub fn parse(self) -> Result<Compiler<Parsed>, CompilerError> {
        //let parser = Parser::new(self.state.tokens)
        //    .parse_statements()?;
        
        Ok(Compiler {
            state: Parsed { /*statements: parser.statements()*/ }
        })
    }
}

impl Compiler<Parsed> {
    pub fn evaluate(self) -> Result<Compiler<Evaluated>, CompilerError> {
        /*let mut intepreter = Intepreter::new();

        for statement in self.state.statements {
            intepreter.execute(&statement)
                .map_err(|error| CompilerError::RuntimeError(error))?;
        }*/
        
        Ok(Compiler {
            state: Evaluated
        })
    }
}
