use std::env;
use std::path::{Path, PathBuf};

use crate::aliases_dirs::{self, AliasesDirs};
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
                    "Failed to resolve default directory for shmarks. Set '{}' environment variable",
                    ENV_VAR_SHMARKS_LIST_PATH
                ))
            })?;
            default_dir.join(SHMARKS_DEFAULT_FILENAME)
        },
    )
}

pub fn parse(shmarks_filepath: &Path) -> Result<AliasesDirs> {
    let toml: toml::Value = {
        let toml_str = util::read_file_contents(&shmarks_filepath)?;

        toml::from_str(&toml_str).map_err(|err| {
            format!(
                "Failed parsing toml from '{}': {}",
                shmarks_filepath.to_str().unwrap(),
                err
            )
        })?
    };
    Ok({
        aliases_dirs::from_toml(&toml).map_err(|err| {
            format!(
                "Failed processing toml from '{}': {}",
                shmarks_filepath.to_str().unwrap(),
                err
            )
        })?
    })
}

pub fn update(shmarks_filepath: &Path, ad: &AliasesDirs) -> Result<()> {
    let updated_shmarks_toml_str = toml::to_string_pretty(&aliases_dirs::to_toml(&ad))?;

    util::replace_contents_of_file(&shmarks_filepath, &updated_shmarks_toml_str).map_err(|e| {
        format!(
            "Failed replacing contents of '{}': {}",
            shmarks_filepath.to_string_lossy(),
            e
        )
    })?;

    Ok(())
}
