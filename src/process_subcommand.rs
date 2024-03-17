use crate::alias_dirs::AliasDirs;
use crate::cli::{LsOpts, NewOpts, RmOpts, SortOpts, Subcommand};
use crate::constants::LS_ALIAS_STYLE_NUMBER_OF_SPACES;
use crate::error::{Error, Result};
use crate::{alias_dirs, constants, util};

pub fn process(subcommand: &Subcommand, ad: &mut AliasDirs) -> Result<()> {
    match subcommand {
        Subcommand::New(opts) => new(&opts, ad)?,
        Subcommand::Rm(opts) => remove(&opts, ad)?,
        Subcommand::Ls(opts) => list(&opts, ad),
        Subcommand::Sort(opts) => sort(&opts, ad),
    }
    Ok(())
}

fn new(opts: &NewOpts, ad: &mut AliasDirs) -> Result<()> {
    if !alias_dirs::alias_name_is_valid(&opts.alias) {
        return Err(Error::from(format!(
            "Validation of alias name '{}' failed. Allowed only alphanumeric characters (letters and digits), underscores, and hyphens.",
            &opts.alias
        )));
    }

    if !opts.force && ad.contains_key(&opts.alias) {
        return Err(Error::AliasAlreadyExists(opts.alias.to_string()));
    }

    let directory = alias_dirs::directory_from_arguments_or_pwd(&opts.directory)?;

    ad.insert(opts.alias.clone(), directory.to_string_lossy().to_string());

    Ok(())
}

fn remove(opts: &RmOpts, ad: &mut AliasDirs) -> Result<()> {
    if let Some(alias) = &opts.alias {
        if !ad.contains_key(alias) {
            return Err(Error::AliasNotFound(alias.to_string()));
        }

        ad.shift_remove(alias);
        return Ok(());
    }

    let directory = alias_dirs::directory_from_arguments_or_pwd(&opts.directory)?;

    alias_dirs::remove_aliases_by_directory(ad, &directory.to_string_lossy())?;

    Ok(())
}

fn list(opts: &LsOpts, ad: &AliasDirs) {
    if ad.keys().len() == 0 {
        return;
    }

    if opts.directory {
        // Colored print in two columns
        alias_dirs::print_keys_long_colored(
            ad,
            &constants::ls_alias_style(),
            LS_ALIAS_STYLE_NUMBER_OF_SPACES,
        );
    } else {
        // Simple print like "a1 a2 a3\n"
        util::print_separated_by_space(ad.keys());
        println!();
    }
}

fn sort(opts: &SortOpts, ad: &mut AliasDirs) {
    if ad.keys().len() == 0 {
        return;
    }

    if opts.directory {
        util::sort_by_value(ad);
    } else {
        util::sort_by_key(ad);
    }
}
