use crate::aliases_dirs::AliasesDirs;
use crate::app;
use crate::constants::{DEFAULT_ALIAS_NAME, LS_COLOR};
use crate::error::{Error, Result};
use crate::{aliases_dirs, normalize, util};
use clap::ArgMatches;
use std::env;
use std::path::{Path, PathBuf};
use std::process;

pub fn ls(m: &ArgMatches, ad: &mut AliasesDirs) {
    let keys_len = ad.keys().len();
    if keys_len == 0 {
        return;
    }

    // Simple print like "a1 a2 a3\n"

    if !m.get_flag(app::ARG_DIRECTORY) {
        util::print_keys_separated_by_space(ad);
        return;
    }

    // Colored print in two columns

    let max_alias_length = ad.keys().map(|s| s.len()).max().unwrap_or(0);

    if max_alias_length == 0 {
        return;
    }

    let alias_style = LS_COLOR.bold();
    let alias_style_len = alias_style.paint(".").to_string().len() - 1;

    const MIN_NUMBER_OF_SPACES: usize = 3;

    let padding = max_alias_length + alias_style_len + MIN_NUMBER_OF_SPACES;
    for alias in ad.keys() {
        println!(
            "{:<width$}{}",
            alias_style.paint(alias).to_string(),
            ad[alias].to_string_lossy(),
            width = padding
        );
    }
}

pub fn rm(m: &ArgMatches, ad: &mut AliasesDirs) -> Result<()> {
    if let Some(alias) = m.get_one::<String>(app::ARG_ALIAS) {
        if ad.contains_key(alias) {
            ad.remove(alias);
            return Ok(());
        }

        return Err(Error::AliasNotFound(alias.to_string()));
    }

    let dir = {
        if let Some(directory) = m.get_one::<PathBuf>(app::ARG_DIRECTORY) {
            normalize::abs_normalize_path(&directory)?
        } else {
            env::current_dir()? // By default current dir is used
        }
    };

    aliases_dirs::remove_elements_of_aliases_dirs_by_value(ad, &dir)?;

    Ok(())
}

pub fn new(m: &ArgMatches, ad: &mut AliasesDirs) -> Result<()> {
    let alias_arg = m.get_one::<String>(app::ARG_ALIAS).unwrap();

    aliases_dirs::validate_alias_name(alias_arg)?;

    let absolute_path_arg = {
        let path_arg = m.get_one::<PathBuf>(app::ARG_DIRECTORY).unwrap();
        normalize::abs_normalize_path(path_arg)?
    };

    ad.insert(alias_arg.to_string(), absolute_path_arg);

    Ok(())
}

pub fn none(m: &ArgMatches, ad: &AliasesDirs, shmarks_file_path: &Path) -> Result<()> {
    if let Some(alias) = m.get_one::<String>(app::ARG_ALIAS) {
        let dir_to_set = ad.get(alias);
        if let Some(dir) = dir_to_set {
            println!("{}", dir.to_string_lossy());
            return Ok(());
        }

        return Err(Error::AliasNotFound(alias.to_string()));
    }

    if m.get_flag(app::ARG_EDIT) {
        let editor = env::var("EDITOR").unwrap_or_else(|_| String::from("vi"));
        process::Command::new(&editor)
            .arg(&shmarks_file_path)
            .status()
            .map_err(|err| {
                format!(
                    "Failed opening file '{}' in editor '{}': {}",
                    shmarks_file_path.to_string_lossy(),
                    editor,
                    err
                )
            })?;

        return Ok(());
    }

    if let Some(dir) = ad.get(DEFAULT_ALIAS_NAME) {
        println!("{}", dir.to_string_lossy());

        return Ok(());
    }

    Err(Error::from(format!(
        "No default directory was set, add alias '{}' to cd into default directory",
        DEFAULT_ALIAS_NAME
    )))
}
