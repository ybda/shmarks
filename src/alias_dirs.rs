use crate::error::{Error, Result};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

pub type AliasDirs = BTreeMap<String, PathBuf>;

pub fn from_toml(toml: &toml::Value) -> Result<AliasDirs> {
    if let toml::Value::Table(table) = toml {
        let mut ad = AliasDirs::new();

        for (key, value) in table {
            if let toml::Value::String(string_value) = value {
                ad.insert(key.to_string(), PathBuf::from(string_value.to_string()));
            }
        }

        Ok(ad)
    } else {
        Err(Error::Msg("Invalid TOML structure".to_string()))
    }
}

pub fn to_toml(ad: &AliasDirs) -> toml::Value {
    let mut table = toml::value::Table::new();

    for (key, value) in ad {
        table.insert(
            key.to_string(),
            toml::Value::String(value.to_str().unwrap().to_string()),
        );
    }

    toml::Value::Table(table)
}

pub fn remove_elements_by_value<P: AsRef<Path>>(ad: &mut AliasDirs, value: P) -> Result<()> {
    let len_before = ad.len();

    {
        let value_str = value.as_ref().to_str().unwrap();
        ad.retain(|_, v| v.to_str().unwrap() != value_str);
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
