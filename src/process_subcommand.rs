use crate::cli::{Cli, LsOpts, NewOpts, RmOpts, SortOpts};
use crate::constants::LS_COLOR;
use crate::error::{Error, Result};
use crate::{alias_dirs, normalize, util};
use std::borrow::Cow;
use crate::alias_dirs::{AliasDirs};

pub fn new(opts: &NewOpts, ad: &mut AliasDirs) -> Result<()> {
    alias_dirs::validate_alias_name(&opts.alias)?;

    if !opts.force && ad.contains_key(&opts.alias) {
        return Err(Error::AliasAlreadyExists(opts.alias.to_string()));
    }

    let dir = if let Some(dir) = &opts.directory {
        Cow::Borrowed(dir)
    } else {
        Cow::Owned(util::retrieve_env_current_dir()?)
    };

    let absolute_path_arg = normalize::abs_normalize_path(dir.as_ref())?;

    ad.insert(opts.alias.clone(), absolute_path_arg.to_str().unwrap().to_string());

    Ok(())
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

    alias_dirs::remove_aliases_by_dir(ad, &dir.to_string_lossy())?;

    Ok(())
}

pub fn ls(opts: &LsOpts, ad: &mut AliasDirs) {
    if ad.keys().len() == 0 {
        return;
    }

    if opts.directory {
        // Colored print in two columns
        alias_dirs::print_keys_long_colored(ad, LS_COLOR.bold(), 3);
    } else {
        // Simple print like "a1 a2 a3\n"
        alias_dirs::print_keys_separated_by_space(ad);
    }
}

pub fn sort(opts: &SortOpts, ad: &mut AliasDirs) {
    if ad.keys().len() == 0 {
        return;
    }

    if opts.directory {
        util::sort_by_value(ad);
    } else {
        util::sort_by_key(ad);
    }
}

pub fn none(cli: &Cli, ad: &AliasDirs) -> Result<()> {
    if let Some(alias) = &cli.alias {
        let dir_to_set = ad.get(alias);
        if let Some(dir) = dir_to_set {
            println!("{}", dir);

            return Ok(());
        }

        return Err(Error::AliasNotFound(alias.to_string()));
    }

    // Shouldn't happen because arg_required_else_help(true) is set
    Err(Error::from("No args were provided"))
}