use std::fs::File;
use std::io;
use std::path::Path;
use sha2::{Sha256, Digest};

use crate::compilation::errors::CompilerError;

pub fn hash_file(path: &Path) -> Result<Vec<u8>, CompilerError> {
    let mut hasher = Sha256::new();
    let mut file = File::open(&path)?;

    io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize().to_vec();

    Ok(hash)
}