use crate::error::Result;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn retrieve_env_current_dir() -> Result<PathBuf> {
    Ok(std::env::current_dir()
        .map_err(|err| format!("Failed retrieving current directory: {}", err))?)
}

pub fn read_file_contents<P: AsRef<Path>>(filepath: P) -> Result<String> {
    let mut buf = String::new();
    let mut file = File::open(filepath.as_ref()).map_err(|err| {
        format!("Failed opening '{}': {}", filepath.as_ref().to_string_lossy(), err)
    })?;
    file.read_to_string(&mut buf).map_err(|err| {
        format!("Failed reading '{}': {}", filepath.as_ref().to_string_lossy(), err)
    })?;
    Ok(buf)
}

