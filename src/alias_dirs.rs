use crate::error::{Error, Result};
use std::path::Path;
use std::fs::File;
use indexmap::IndexMap;
use crate::util;
use std::io::Write;
use nu_ansi_term::Style;

pub type AliasDirs = IndexMap<String, String>;

pub fn ad_from_file<P: AsRef<Path>>(shmarks_filepath: P) -> Result<AliasDirs> {
    let toml_str = util::read_file_contents(&shmarks_filepath)?;

    Ok(toml::from_str(&toml_str).map_err(|err| {
        format!(
            "Failed parsing toml from '{}': {}",
            shmarks_filepath.as_ref().to_str().unwrap(),
            err
        )
    })?)
}

pub fn update_shmarks_file<P: AsRef<Path>>(shmarks_filepath: P, ad: &AliasDirs) -> Result<()> {
    // Truncate file
    let mut truncated_file = File::create(shmarks_filepath.as_ref())
        .map_err(|err| format!("Failed truncating '{}': {}", shmarks_filepath.as_ref().to_string_lossy(), err))?;

    let updated_shmarks_toml_str = toml::to_string_pretty(&ad)?;

    truncated_file.write_all(updated_shmarks_toml_str.as_bytes()).map_err(|err| {
        format!("Failed writing into '{}': {}", shmarks_filepath.as_ref().to_string_lossy(), err)
    })?;

    Ok(())
}

pub fn remove_aliases_by_dir(ad: &mut AliasDirs, val: &str) -> Result<()> {
    let len_before = ad.len();

    ad.retain(|_, v| v != val);

    if len_before == ad.len() {
        return Err(Error::AliasOfDirectoryXNotFound(val.to_string()));
    }
    Ok(())
}

pub fn print_keys_separated_by_space(map: &AliasDirs) {
    let mut first = true;
    for key in map.keys() {
        if first {
            print!("{}", key);
            first = false;
        } else {
            print!(" {}", key);
        }
    }
    println!();
}

pub fn print_keys_long_colored(map: &AliasDirs, key_style: Style, min_number_of_spaces: usize) {
    let padding = {
        let max_key_length = map.keys().map(|s| s.len()).max().unwrap_or(0);
        let key_style_len = key_style.paint(".").to_string().len() - 1; // minus one because we don't count the dot
        max_key_length + key_style_len + min_number_of_spaces
    };

    for (key, val) in map {
        println!(
            "{:<width$}{}",
            key_style.paint(key).to_string(),
            val,
            width = padding
        );
    }
}

pub fn validate_alias_name(alias_name: &str) -> Result<()> {
    let pattern = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();

    if !pattern.is_match(alias_name) {
        return Err(Error::Msg(format!(
            "Alias name is invalid: '{}'", alias_name
        )));
    }

    Ok(())
}

pub fn sort_by_key(ad: &mut AliasDirs) {
    ad.sort_by(|k1, _, k2, _| k1.cmp(k2));
}

pub fn sort_by_value(ad: &mut AliasDirs) {
    ad.sort_by(|_, value1, _, value2| value1.cmp(value2));
}

