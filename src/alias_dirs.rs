use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use nu_ansi_term::Style;

use crate::cli::Cli;
use crate::error::{Error, Result};
use crate::{normalize, shmarks_warning, util};

pub type AliasDirs = IndexMap<String, String>;

pub fn ad_from_file<P: AsRef<Path>>(shmarks_filepath: P) -> Result<AliasDirs> {
    if !shmarks_filepath.as_ref().exists() {
        File::create(shmarks_filepath.as_ref())
            .map_err(|err| format!("Failed creating file: {}", err))?;
    }

    let toml_str = util::read_file_contents(shmarks_filepath.as_ref())
        .map_err(|err| format!("Failed retrieving file contents: {}", err))?;

    Ok(toml::from_str(&toml_str).map_err(|err| format!("Failed parsing toml: {}", err))?)
}

pub fn update_shmarks_file<P: AsRef<Path>>(shmarks_filepath: P, ad: &AliasDirs) -> Result<()> {
    // Truncate file
    let mut truncated_file = File::create(shmarks_filepath.as_ref())
        .map_err(|err| format!("Failed truncating: {}", err))?;

    let updated_shmarks_toml_str = toml::to_string_pretty(&ad)?;

    truncated_file
        .write_all(updated_shmarks_toml_str.as_bytes())
        .map_err(|err| format!("Failed writing: {}", err))?;

    Ok(())
}

pub fn process_directory_jump(cli: &Cli, ad: &AliasDirs) -> Result<()> {
    let alias = match &cli.alias {
        Some(a) => a,
        None => {
            panic!(
                "No arguments provided. Shouldn't happen because arg_required_else_help(true) is set"
            )
        }
    };

    let directory_of_alias = match ad.get(alias) {
        Some(d) => d,
        None => {
            return Err(Error::AliasNotFound(alias.to_string()));
        }
    };

    println!("{}", directory_of_alias);

    return Ok(());
}

pub fn remove_aliases_by_directory(ad: &mut AliasDirs, directory: &str) -> Result<()> {
    let len_before = ad.len();

    ad.retain(|_, v| v != directory);

    if len_before == ad.len() {
        shmarks_warning!("Alias of directory '{}' not found", directory.to_string());
    }

    Ok(())
}

pub fn print_keys_long_colored(ad: &AliasDirs, key_style: &Style, min_number_of_spaces: usize) {
    let padding = {
        let max_key_length = ad.keys().map(|s| s.len()).max().unwrap_or(0);
        let key_style_len = key_style.paint(".").to_string().len() - 1; // minus one because we don't count the dot
        max_key_length + key_style_len + min_number_of_spaces
    };

    for (key, val) in ad {
        // don't use `println!` to avoid overhead of flushing each time
        print!("{:<width$}{}\n", key_style.paint(key).to_string(), val, width = padding);
    }
}

/// Accept only alphanumeric characters (letters and digits), underscores, and
/// hyphens. Length must be > zero.
pub fn alias_name_is_valid(alias_name: &str) -> bool {
    let pattern = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    pattern.is_match(alias_name)
}
