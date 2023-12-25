#![feature(absolute_path)]

mod aliases_dirs;
mod app;
mod error;
mod normalize;
mod process_subcommand;
mod util;
use crate::aliases_dirs::AliasesDirs;
use crate::error::Result;
use dirs;
use error::Error;
use std::env;
use std::path::PathBuf;
use std::process;
use toml;

fn main() {
    run().unwrap_or_else(|e| {
        let stderr = std::io::stderr();
        error::default_error_handler(&e, &mut stderr.lock());
        process::exit(1);
    })
}

fn run() -> Result<()> {
    let shmarks_file_path = retrieve_shmarks_file_path()?;

    let mut ad: AliasesDirs = retrieve_aliases_dirs(&shmarks_file_path)?;

    match_subcommands(&mut ad, &shmarks_file_path)?;

    Ok(())
}

fn retrieve_shmarks_file_path() -> Result<PathBuf> {
    const ENV_VAR_NAME: &str = "SHMARKS_LIST_FILE";

    let shmarks_file_path = match env::var(ENV_VAR_NAME) {
        Ok(value) => PathBuf::from(value),
        Err(_) => dirs::config_local_dir().ok_or_else(|| {
            Error::Msg(format!(
                "Failed to resolve default config directory. Set '{}' environment variable",
                ENV_VAR_NAME
            ))
        })?,
    };

    if !shmarks_file_path.is_file() {
        return Err(Error::Msg(format!(
            "Environment variable '{}' is not a file, provided path: {}",
            ENV_VAR_NAME,
            shmarks_file_path.to_string_lossy()
        )));
    }

    Ok(shmarks_file_path)
}

fn retrieve_aliases_dirs(shmarks_file_path: &PathBuf) -> Result<AliasesDirs> {
    let toml_str = util::read_contents_file(&shmarks_file_path).map_err(|err| {
        format!(
            "Failed reading '{}': {}",
            shmarks_file_path.to_str().unwrap(),
            err
        )
    })?;

    let toml: toml::Value = toml::from_str(&toml_str).map_err(|err| {
        format!(
            "Failed parsing toml from '{}': {}",
            shmarks_file_path.to_str().unwrap(),
            err
        )
    })?;

    let ad: AliasesDirs = aliases_dirs::from_toml(&toml).map_err(|err| {
        format!(
            "Failed processing toml from '{}': {}",
            shmarks_file_path.to_str().unwrap(),
            err
        )
    })?;

    Ok(ad)
}

fn match_subcommands(ad: &mut AliasesDirs, shmarks_file_path: &PathBuf) -> Result<()> {
    let matches = app::matches();

    match matches.subcommand() {
        Some((app::SUBCOMMAND_NEW, sub_m)) => process_subcommand::new(&sub_m, ad)?,
        Some((app::SUBCOMMAND_RM, sub_m)) => process_subcommand::rm(&sub_m, ad)?,
        Some((app::SUBCOMMAND_LS, sub_m)) => process_subcommand::ls(&sub_m, ad),
        _ => process_subcommand::none(&matches, &ad, &shmarks_file_path)?,
    }

    if let Some(name) = matches.subcommand_name() {
        if name == app::SUBCOMMAND_NEW || name == app::SUBCOMMAND_RM {
            update_toml_file(&ad, &shmarks_file_path)?;
        }
    }

    Ok(())
}

fn update_toml_file(ad: &AliasesDirs, shmarks_file_path: &PathBuf) -> Result<()> {
    let toml_new_string = toml::to_string_pretty(&aliases_dirs::to_toml(&ad))?;

    util::replace_contents_of_file(&shmarks_file_path, &toml_new_string).map_err(|e| {
        format!(
            "Failed replacing contents of '{}': {}",
            shmarks_file_path.to_string_lossy(),
            e
        )
    })?;

    Ok(())
}
