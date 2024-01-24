#![feature(absolute_path)]

use clap::Parser;
mod alias_dirs;
mod cli;
mod constants;
mod error;
mod normalize;
mod process_subcommand;
mod util;
use std::path::{Path, PathBuf};
use std::{env, process};

use crate::alias_dirs::AliasDirs;
use crate::cli::{Cli, Commands};
use crate::constants::{
    ENV_VAR_SHMARKS_AUTO_SORT, ENV_VAR_SHMARKS_LIST_PATH, SHMARKS_DEFAULT_FILENAME,
};
use crate::error::{Error, Result};

fn main() {
    run().unwrap_or_else(|e| {
        error::default_error_handler(&e, &mut std::io::stderr().lock());
        process::exit(1);
    })
}

fn run() -> Result<()> {
    let shmarks_filepath = retrieve_shmarks_filepath()?;

    let mut ad = alias_dirs::ad_from_file(&shmarks_filepath).map_err(|err| {
        format!("Failed processing toml file '{}': {}", &shmarks_filepath.to_string_lossy(), err)
    })?;

    let opts = Cli::parse();

    process_args(&opts, &mut ad, &shmarks_filepath)?;

    Ok(())
}

fn retrieve_shmarks_filepath() -> Result<PathBuf> {
    Ok(if let Some(fp) = env::var_os(ENV_VAR_SHMARKS_LIST_PATH).map(PathBuf::from) {
        fp
    } else {
        let default_dir = dirs::data_local_dir().ok_or_else(|| {
            Error::from(format!(
                "Failed resolving default directory for shmarks. Set '{}' environment variable",
                ENV_VAR_SHMARKS_LIST_PATH
            ))
        })?;
        default_dir.join(SHMARKS_DEFAULT_FILENAME)
    })
}

fn process_args<P: AsRef<Path>>(opts: &Cli, ad: &mut AliasDirs, shmarks_filepath: P) -> Result<()> {
    let subcommand = match &opts.command {
        Some(s) => {
            process_subcommand::process(s, ad)?;
            s
        }
        None => {
            alias_dirs::process_directory_jump(&opts, ad)?;
            return Ok(());
        }
    };

    if matches!(subcommand, Commands::New(_) | Commands::Rm(_)) {
        try_auto_sort(ad);
    }

    if matches!(subcommand, Commands::New(_) | Commands::Rm(_) | Commands::Sort(_)) {
        alias_dirs::update_shmarks_file(shmarks_filepath.as_ref(), ad).map_err(|err| {
            format!(
                "Failed updating shmarks file '{}': {}",
                shmarks_filepath.as_ref().to_string_lossy(),
                err
            )
        })?;
    }

    Ok(())
}

pub fn try_auto_sort(ad: &mut AliasDirs) {
    let var = match env::var(ENV_VAR_SHMARKS_AUTO_SORT) {
        Ok(val) => val,
        Err(_) => {
            return;
        }
    };

    if var == "a" {
        util::sort_by_key(ad)
    } else if var == "d" {
        util::sort_by_value(ad)
    }
}
