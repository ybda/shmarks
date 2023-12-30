use crate::error::Result;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use nu_ansi_term::Style;

pub fn retrieve_env_current_dir() -> Result<PathBuf> {
    Ok(std::env::current_dir()
        .map_err(|err| format!("Failed retrieving current directory: {}", err))?)
}

pub fn read_file_contents<P: AsRef<Path>>(filepath: P) -> Result<String> {
    let mut str = String::new();
    let mut file = File::open(filepath.as_ref()).map_err(|err| {
        format!("Failed opening '{}': {}", filepath.as_ref().to_string_lossy(), err)
    })?;
    file.read_to_string(&mut str).map_err(|err| {
        format!("Failed reading '{}': {}", filepath.as_ref().to_string_lossy(), err)
    })?;
    Ok(str)
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

pub fn print_keys_long_colored<P: AsRef<Path>>(map: &BTreeMap<String, P>, key_style: Style, min_number_of_spaces: usize) {
    let padding = {
        let max_key_length = map.keys().map(|s| s.len()).max().unwrap_or(0);
        let key_style_len = key_style.paint(".").to_string().len() - 1;
        max_key_length + key_style_len + min_number_of_spaces
    };

    for alias in map.keys() {
        println!(
            "{:<width$}{}",
            key_style.paint(alias).to_string(),
            map[alias].as_ref().to_string_lossy(),
            width = padding
        );
    }
}