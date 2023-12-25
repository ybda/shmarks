use crate::error::{Error, Result};
use std::{collections::BTreeMap, path::{PathBuf, Path}};

pub type AliasesDirs = BTreeMap<String, PathBuf>;

pub fn from_toml(toml: &toml::Value) -> Result<AliasesDirs> {
    if let toml::Value::Table(table) = toml {
        let mut ad: AliasesDirs = BTreeMap::new();

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

pub fn to_toml(ad: &AliasesDirs) -> toml::Value {
    let mut table = toml::value::Table::new();

    for (key, value) in ad {
        table.insert(
            key.to_string(),
            toml::Value::String(value.to_str().unwrap().to_string()),
        );
    }

    toml::Value::Table(table)
}

pub fn remove_elements_of_aliases_dirs_by_value(
    ad: &mut AliasesDirs,
    value: &Path,
) -> Result<()> {
    let len_before = ad.len();

    {
        let value_str = value.to_str().unwrap();
        ad.retain(|_, v| v.to_str().unwrap() != value_str);
    }

    if len_before == ad.len() {
        return Err(Error::AliasOfDirectoryXNotFound(
            value.to_string_lossy().to_string(),
        ));
    }

    Ok(())
}

pub fn validate_alias_name(alias_name: &str) -> Result<()> {
    let pattern = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();

    if !pattern.is_match(alias_name) {
        return Err(Error::Msg(format!("Alias name is invalid: '{}'", alias_name)));
    }

    Ok(())
}