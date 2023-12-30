#![feature(absolute_path)]

mod aliases_dirs;
mod app;
mod constants;
mod error;
mod normalize;
mod process_subcommand;
mod util;
use crate::aliases_dirs::AliasesDirs;
use crate::constants::{ENV_VAR_SHMARKS_LIST_PATH, SHMARKS_DEFAULT_FILENAME};
use crate::error::Result;
use dirs;
use error::Error;
use std::env;
use std::path::{Path, PathBuf};
use std::process;
use toml;

fn main() {
    run().unwrap_or_else(|e| {
        error::default_error_handler(&e, &mut std::io::stderr().lock());
        process::exit(1);
    })
}

fn run() -> Result<()> {
    let shmarks_filepath = if let Some(fp) =
        env::var_os(ENV_VAR_SHMARKS_LIST_PATH).map(PathBuf::from)
    {
        fp
    } else {
        let default_dir = dirs::data_local_dir().ok_or_else(|| {
            Error::from(format!(
                "Failed to resolve default directory for shmarks. Set '{}' environment variable",
                ENV_VAR_SHMARKS_LIST_PATH
            ))
        })?;
        default_dir.join(SHMARKS_DEFAULT_FILENAME)
    };

    util::create_file_if_not_exists(&shmarks_filepath)?;

    let mut ad: AliasesDirs = retrieve_aliases_dirs(&shmarks_filepath)?;

    process_args(&mut ad, &shmarks_filepath)?;

    Ok(())
}

fn retrieve_aliases_dirs(shmarks_file_path: &Path) -> Result<AliasesDirs> {
    Ok({
        let toml: toml::Value = {
            let toml_str = util::read_file_contents(&shmarks_file_path).map_err(|err| {
                format!(
                    "Failed reading '{}': {}",
                    shmarks_file_path.to_str().unwrap(),
                    err
                )
            })?;

            toml::from_str(&toml_str).map_err(|err| {
                format!(
                    "Failed parsing toml from '{}': {}",
                    shmarks_file_path.to_str().unwrap(),
                    err
                )
            })?
        };

        aliases_dirs::from_toml(&toml).map_err(|err| {
            format!(
                "Failed processing toml from '{}': {}",
                shmarks_file_path.to_str().unwrap(),
                err
            )
        })?
    })
}

fn process_args(ad: &mut AliasesDirs, shmarks_file_path: &Path) -> Result<()> {
    fn update_shmarks_file(ad: &AliasesDirs, shmarks_file_path: &Path) -> Result<()> {
        let updated_shmarks_toml_str = toml::to_string_pretty(&aliases_dirs::to_toml(&ad))?;

        util::replace_contents_of_file(&shmarks_file_path, &updated_shmarks_toml_str).map_err(
            |e| {
                format!(
                    "Failed replacing contents of '{}': {}",
                    shmarks_file_path.to_string_lossy(),
                    e
                )
            },
        )?;

        Ok(())
    }

    let matches = app::matches();

    match matches.subcommand() {
        Some((app::SUBCOMMAND_NEW, sub_m)) => {
            process_subcommand::new(&sub_m, ad)?;
            update_shmarks_file(ad, shmarks_file_path)?;
        }
        Some((app::SUBCOMMAND_RM, sub_m)) => {
            process_subcommand::rm(&sub_m, ad)?;
            update_shmarks_file(ad, shmarks_file_path)?;
        }
        Some((app::SUBCOMMAND_LS, sub_m)) => process_subcommand::ls(&sub_m, ad),
        _ => process_subcommand::none(&matches, &ad, &shmarks_file_path)?,
    }

    Ok(())
}
