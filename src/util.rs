use crate::error::Result;
use crate::toml_parser::AliasesDirs;
use std::collections::BTreeMap;
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

pub fn remove_first_value_from_aliases_dirs(
    map: &mut AliasesDirs,
    value: &PathBuf,
) -> std::result::Result<(), std::io::Error> {
    match remove_first_value_from_map(map, value) {
        Ok(_) => return Ok(()),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                return Err(std::io::Error::new(e.kind(), format!("Alias of directory '{}' not found", value.to_string_lossy())))
            }
            _ => return Err(e),
        },
    }
}

fn remove_first_value_from_map<K, V>(
    map: &mut BTreeMap<K, V>,
    value: &V,
) -> std::result::Result<(), std::io::Error>
where
    V: PartialEq,
    K: Ord + Clone,
{
    let key_to_remove = map
        .iter()
        .find_map(|(k, v)| if *v == *value { Some(k.clone()) } else { None });

    if let Some(key) = key_to_remove {
        map.remove(&key);
        return Ok(());
    }

    return Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Key wasn't found",
    ));
}
