use std::fs::{self, File};
use std::io::Write;
use std::rc::Rc;
use serde::{Deserialize, Serialize};

use crate::projects::project::Project;
use crate::compilation::errors::CompilerError;
use crate::compilation::source::Source;
use crate::compilation::token::Token;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Intermediate {
    pub hash: Rc<[u8]>,
    pub tokens : Rc<[Token]>,
}

impl Intermediate {
    pub fn new(tokens: &[Token], hash: Rc<[u8]>) -> Self {
        Self {
            hash: hash,
            tokens: tokens.into(),
        }
    }

    pub fn save_for(&self, source: &Source) -> Result<(), CompilerError> {
        let intermediate_directory = Project::get_intermediate_dir(true)?;
        let mut full_path = intermediate_directory.join(source.path.as_ref());
        full_path.set_extension("prt");

        let bytes = bincode::serialize(self)?;
        let mut file = File::create(full_path)?;
        file.write_all(&bytes)?;

        Ok(())
    }
}

impl TryFrom<&Source> for Intermediate {
    type Error = CompilerError;

    fn try_from(source: &Source) -> Result<Self, Self::Error> {
        let intermediate_directory = Project::get_intermediate_dir(false)?;
        let mut full_path = intermediate_directory.join(source.path.as_ref());
        full_path.set_extension("prt");

        let bytes = fs::read(full_path)?;
        let intermediate = bincode::deserialize::<Intermediate>(&bytes)?;

        if source.hash == intermediate.hash {
            Ok(intermediate)
        } else {
            Err(CompilerError::None)
        }
    }
}
