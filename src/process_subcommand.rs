use crate::aliases_dirs::AliasesDirs;
use crate::cli::{Cli, LsOpts, NewOpts, RmOpts};
use crate::constants::LS_COLOR;
use crate::error::{Error, Result};
use crate::{aliases_dirs, normalize, util};
use std::borrow::Cow;

pub fn ls(opts: &LsOpts, ad: &mut AliasesDirs) {
    if ad.keys().len() == 0 {
        return;
    }

    // Simple print like "a1 a2 a3\n"

    if !opts.directory {
        util::print_keys_separated_by_space(ad);
        return;
    }

    // Colored print in two columns

    let max_alias_length = ad.keys().map(|s| s.len()).max().unwrap_or(0);

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

pub fn rm(opts: &RmOpts, ad: &mut AliasesDirs) -> Result<()> {
    if let Some(alias) = &opts.alias {
        if ad.contains_key(alias) {
            ad.remove(alias);
            return Ok(());
        }

        return Err(Error::AliasNotFound(alias.to_string()));
    }

    let dir = {
        if let Some(dir) = &opts.directory {
            normalize::abs_normalize_path(&dir)?
        } else {
            // By default current dir is used
            util::current_dir()?
        }
    };

    aliases_dirs::remove_elements_by_value(ad, &dir)?;

    Ok(())
}

pub fn new(opts: &NewOpts, ad: &mut AliasesDirs) -> Result<()> {
    aliases_dirs::validate_alias_name(&opts.alias)?;

    let dir = if let Some(dir) = &opts.directory {
        Cow::Borrowed(dir)
    } else {
        Cow::Owned(util::current_dir()?)
    };

    let absolute_path_arg = { normalize::abs_normalize_path(dir.as_ref())? };

    ad.insert(opts.alias.clone(), absolute_path_arg);

    Ok(())
}

pub fn none(cli: &Cli, ad: &AliasesDirs) -> Result<()> {
    if let Some(alias) = &cli.alias {
        let dir_to_set = ad.get(alias);
        if let Some(dir) = dir_to_set {
            println!("{}", dir.to_string_lossy());

            return Ok(());
        }

        return Err(Error::AliasNotFound(alias.to_string()));
    }

    // Shouldn't happen because arg_required_else_help(true) is set
    Err(Error::from(format!("No args were provided")))
}
