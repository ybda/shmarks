use crate::error::{Error, Result};
use crate::toml_parser::AliasesDirs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn read_contents_file(filepath: &PathBuf) -> Result<String> {
    let mut toml_str = String::new();
    let mut file = File::open(filepath)?;
    file.read_to_string(&mut toml_str)?;
    Ok(toml_str)
}

pub fn replace_contents_of_file(path: &PathBuf, contents: &str) -> Result<()> {
    let mut modified_file = File::create(path)?; // Truncate file
    modified_file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn remove_elements_of_aliases_dirs_by_value(
    map: &mut AliasesDirs,
    value: &PathBuf,
) -> Result<()> {
    let len_before = map.len();

    {
        let value_str = value.to_str().unwrap();
        map.retain(|_, v| v.to_str().unwrap() != value_str);
    }

    if len_before == map.len() {
        return Err(Error::AliasOfDirectoryXNotFound(
            value.to_string_lossy().to_string(),
        ));
    }

    Ok(())
}
