use crate::alias_dirs::AliasDirs;
use crate::cli::{Cli, LsOpts, NewOpts, RmOpts};
use crate::constants::LS_COLOR;
use crate::error::{Error, Result};
use crate::{alias_dirs, normalize, util};
use std::borrow::Cow;

pub fn ls(opts: &LsOpts, ad: &mut AliasDirs) {
    if ad.keys().len() == 0 {
        return;
    }

    if opts.directory {
        // Colored print in two columns
        util::print_keys_long_colored(ad, LS_COLOR.bold(), 3);
    } else {
        // Simple print like "a1 a2 a3\n"
        util::print_keys_separated_by_space(ad);
    }
}

pub fn rm(opts: &RmOpts, ad: &mut AliasDirs) -> Result<()> {
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
            util::retrieve_env_current_dir()?
        }
    };

    alias_dirs::remove_elements_by_value(ad, &dir)?;

    Ok(())
}

pub fn new(opts: &NewOpts, ad: &mut AliasDirs) -> Result<()> {
    alias_dirs::validate_alias_name(&opts.alias)?;

    let dir = if let Some(dir) = &opts.directory {
        Cow::Borrowed(dir)
    } else {
        Cow::Owned(util::retrieve_env_current_dir()?)
    };

    let absolute_path_arg = normalize::abs_normalize_path(dir.as_ref())?;

    ad.insert(opts.alias.clone(), absolute_path_arg);

    Ok(())
}

pub fn none(cli: &Cli, ad: &AliasDirs) -> Result<()> {
    if let Some(alias) = &cli.alias {
        let dir_to_set = ad.get(alias);
        if let Some(dir) = dir_to_set {
            println!("{}", dir.to_string_lossy());

            return Ok(());
        }

        return Err(Error::AliasNotFound(alias.to_string()));
    }

    // Shouldn't happen because arg_required_else_help(true) is set
    Err(Error::from("No args were provided"))
}
