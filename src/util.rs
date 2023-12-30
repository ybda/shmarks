use crate::error::Result;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub fn current_dir() -> Result<PathBuf> {
    Ok(std::env::current_dir()
        .map_err(|err| format!("Failed retrieving current directory: {}", err))?)
}

pub fn read_file_contents(filepath: &Path) -> Result<String> {
    let mut toml_str = String::new();
    let mut file = File::open(filepath)?;
    file.read_to_string(&mut toml_str)?;
    Ok(toml_str)
}

pub fn replace_contents_of_file(path: &Path, contents: &str) -> Result<()> {
    let mut modified_file = File::create(path)?; // Truncate file
    modified_file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn create_file_if_not_exists(filepath: &Path) -> Result<()> {
    if !filepath.exists() {
        File::create(&filepath)
            .map_err(|err| format!("Failed creating '{}': {}", &filepath.to_string_lossy(), err))?;
    }

    Ok(())
}

pub fn print_keys_separated_by_space<K, V>(map: &BTreeMap<K, V>)
where
    K: std::fmt::Display,
{
    let mut first = true;
    for key in map.keys() {
        if first {
            print!("{}", key);
            first = false;
        } else {
            print!(" {}", key);
        }
    }
    println!();
}
