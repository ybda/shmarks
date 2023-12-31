use crate::error::{Error, Result};
use std::{
    path::{Path},
};
use toml::{Table, Value};


pub fn remove_elements_by_value<P: AsRef<Path>>(ad: &mut Table, value: P) -> Result<()> {
    let len_before = ad.len();

    {
        let value_str = value.as_ref().to_str().unwrap();
        ad.retain(|_, v| v.is_str() && v.as_str().unwrap() != value_str);
    }

    if len_before == ad.len() {
        return Err(Error::AliasOfDirectoryXNotFound(
            value.as_ref().to_string_lossy().to_string(),
        ));
    }

    Ok(())
}

pub fn validate_alias_name(alias_name: &str) -> Result<()> {
    let pattern = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();

    if !pattern.is_match(alias_name) {
        return Err(Error::Msg(format!(
            "Alias name is invalid: '{}'",
            alias_name
        )));
    }

    Ok(())
}
