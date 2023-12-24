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

    fn check(before: &str, after: &str) {
        println!("-----------------\nnormalizing {:?}", before);
        // The test here doesn't work on Windows
        //
        // There are two problems, at least:
        //
        // * strings used for test use the '/' separator. This is a test problem
        // * we do a "end with '/'" test in the tested function. This might
        //   lead to suboptimal interaction on windows
        assert_eq!(
            normalize_path(&PathBuf::from(before)).to_string_lossy(),
            after
        );
    }

    #[test]
    fn test_resolve_nonexistent_path() {
        check("/home/dys/..", "/home");
        check("/home/./dys/../", "/home");
        check("./home/./dys/../", "home");
        check("/abc/test/../thing.png", "/abc/thing.png");
        check("/abc/def/../../thing.png", "/thing.png");
        check("/abc/def/../fe/../thing.png", "/abc/thing.png");
    }
}
