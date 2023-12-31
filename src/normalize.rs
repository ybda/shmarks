use crate::error::Result;
use std::path::{Path, PathBuf};

pub fn abs_normalize_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    Ok(normalize_path(std::path::absolute(path)?))
}

fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.as_ref().components() {
        match component {
            std::path::Component::Prefix(_c) => normalized.push(component),
            std::path::Component::RootDir => normalized.push(component),
            std::path::Component::Normal(_c) => normalized.push(component),
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            _ => {}
        }
    }
    normalized
}

#[cfg(test)]
mod tests {
    use super::*;

    fn convert_to_windows_path(path: &str) -> String {
        if cfg!(windows) {
            path.replace("/", "\\")
        } else {
            path.to_string()
        }
    }

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
            ("C:/abc/def/../fe/../thing.png", "C:/abc/thing.png"),
        ];

        for &(left, right) in &test_cases {
            assert_eq!(
                normalize_path(&PathBuf::from(convert_to_windows_path(left))).to_string_lossy(),
                convert_to_windows_path(right)
            );
        }
    }

    #[test]
    fn test_abs_normalize_path() {
        let paths_eq: Vec<(&str, &str)> = vec![
            ("/home/dys/..", "/home"),
            ("/home/./dys/../", "/home"),
            ("/abc/test/../thing.png", "/abc/thing.png"),
            ("/abc/def/../../thing.png", "/thing.png"),
            ("/abc/def/../fe/../thing.png", "/abc/thing.png"),
        ];

        let paths_ends_with = vec![
            ("home/./dys/../", "/home"),
            ("./home/./dys/../", "/home"),
            ("abc/test/../thing.png", "/abc/thing.png"),
        ];

        for &(left, right) in &paths_eq {
            assert_eq!(
                abs_normalize_path(&PathBuf::from(left)).unwrap().to_string_lossy(),
                right
            );
        }

        for &(left, right) in &paths_ends_with {
            let actual_val = abs_normalize_path(PathBuf::from(left)).unwrap();

            assert!(
                actual_val.to_string_lossy().ends_with(right)
            );
        }
    }
}
