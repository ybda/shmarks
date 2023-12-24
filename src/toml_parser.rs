use crate::error::{Error, Result};
use std::{collections::BTreeMap, path::PathBuf};

pub type AliasesDirs = BTreeMap<String, PathBuf>;

pub fn to_btreemap(toml: &toml::Value) -> Result<AliasesDirs> {
    if let toml::Value::Table(table) = toml {
        let mut btreemap: AliasesDirs = BTreeMap::new();

        for (key, value) in table {
            if let toml::Value::String(string_value) = value {
                btreemap.insert(key.to_string(), PathBuf::from(string_value.to_string()));
            }
        }

        Ok(btreemap)
    } else {
        Err(Error::Msg("Invalid TOML structure".to_string()))
    }
}

pub fn to_toml(string_map: &AliasesDirs) -> toml::Value {
    let mut table = toml::value::Table::new();

    for (key, value) in string_map {
        table.insert(
            key.to_string(),
            toml::Value::String(value.to_str().unwrap().to_string()),
        );
    }

    toml::Value::Table(table)
}
