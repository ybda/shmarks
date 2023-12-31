use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::Write;
use toml::Table;

use crate::alias_dirs::{self};
use crate::constants::{ENV_VAR_SHMARKS_LIST_PATH, SHMARKS_DEFAULT_FILENAME};
use crate::error::{Error, Result};
use crate::util;

pub fn retrieve_filepath() -> Result<PathBuf> {
    Ok(
        if let Some(fp) = env::var_os(ENV_VAR_SHMARKS_LIST_PATH).map(PathBuf::from) {
            fp
        } else {
            let default_dir = dirs::data_local_dir().ok_or_else(|| {
                Error::from(format!(
                    "Failed resolving default directory for shmarks. Set '{}' environment variable",
                    ENV_VAR_SHMARKS_LIST_PATH
                ))
            })?;
            default_dir.join(SHMARKS_DEFAULT_FILENAME)
        },
    )
}

pub fn toml_from_file<P: AsRef<Path>>(shmarks_filepath: P) -> Result<Table> {
    let toml_str = util::read_file_contents(&shmarks_filepath)?;

    Ok(toml::from_str(&toml_str).map_err(|err| {
        format!(
            "Failed parsing toml from '{}': {}",
            shmarks_filepath.as_ref().to_str().unwrap(),
            err
        )
    })?)
}

pub fn update<P: AsRef<Path>>(shmarks_filepath: P, ad: &Table) -> Result<()> {
    // Truncate file
    let mut truncated_file = File::create(shmarks_filepath.as_ref())
        .map_err(|err| format!("Failed truncating '{}': {}", shmarks_filepath.as_ref().to_string_lossy(), err))?;

    let updated_shmarks_toml_str = toml::to_string_pretty(&ad)?;

    truncated_file.write_all(updated_shmarks_toml_str.as_bytes()).map_err(|err| {
        format!("Failed writing into '{}': {}", shmarks_filepath.as_ref().to_string_lossy(), err)
    })?;

    Ok(())
}
