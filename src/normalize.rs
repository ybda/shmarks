use crate::error::Result;
use std::path::{Path, PathBuf};

pub fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut normalized = if path.as_ref().is_absolute() {
        PathBuf::from("/")
    } else {
        PathBuf::new()
    };
    for component in path.as_ref().components() {
        match component {
            std::path::Component::Normal(c) => normalized.push(c),
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            _ => {}
        }
    }
    normalized
}

pub fn abs_normalize_path(path: &PathBuf) -> Result<PathBuf> {
    let path_str = path.to_str().unwrap();
    if path_str == "." || path_str == ".." {
        return Ok(std::fs::canonicalize(path)?);
    }
    Ok(normalize_path(std::path::absolute(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path() {
        let test_cases: Vec<(&str, &str)> = vec![
            ("/home/dys/..", "/home"),
            ("/home/./dys/../", "/home"),
            ("home/./dys/../", "home"),
            ("./home/./dys/../", "home"),
            ("/abc/test/../thing.png", "/abc/thing.png"),
            ("/abc/def/../../thing.png", "/thing.png"),
            ("/abc/def/../fe/../thing.png", "/abc/thing.png"),
        ];

        for &(left, right) in &test_cases {
            assert_eq!(
                normalize_path(&PathBuf::from(left)).to_string_lossy(),
                right
            );
        }
    }
}
