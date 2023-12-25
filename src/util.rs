use crate::error::Result;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

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
