use std::fmt::Display;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::path::Path;

use indexmap::IndexMap;

use crate::error::Result;

pub fn read_file_contents<P: AsRef<Path>>(filepath: P) -> Result<String> {
    let mut buf = String::new();
    let mut file =
        File::open(filepath.as_ref()).map_err(|err| format!("Failed opening: {}", err))?;
    file.read_to_string(&mut buf).map_err(|err| format!("Failed reading: {}", err))?;
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

pub fn print_separated_by_space<T, I>(mut iter: I)
where
    T: Display,
    I: Iterator<Item = T>,
{
    if let Some(first) = iter.next() {
        print!("{}", first);
    }
    for item in iter {
        print!(" {}", item);
    }
}
