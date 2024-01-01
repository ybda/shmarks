use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::path::{Path, PathBuf};

use indexmap::IndexMap;

use crate::error::Result;

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

pub fn sort_by_key<K, V>(m: &mut IndexMap<K, V>)
where
    K: Ord + Hash,
{
    m.sort_by(|k1, _, k2, _| k1.cmp(k2));
}

pub fn sort_by_value<K, V>(m: &mut IndexMap<K, V>)
where
    K: Ord + Hash,
    V: Ord,
{
    m.sort_by(|_, v1, _, v2| v1.cmp(v2));
}
