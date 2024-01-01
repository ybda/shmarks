#![feature(absolute_path)]

use std::fs::File;

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

    if !shmarks_filepath.exists() {
        File::create(&shmarks_filepath).map_err(|err| {
            format!("Failed creating '{}': {}", &shmarks_filepath.to_string_lossy(), err)
        })?;
    }

    let mut ad = alias_dirs::ad_from_file(&shmarks_filepath)?;

    process_args(&mut ad, &shmarks_filepath)?;

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

fn process_args<P: AsRef<Path>>(ad: &mut AliasDirs, shmarks_filepath: P) -> Result<()> {
    let opts = Cli::parse();

    match opts.command {
        Some(Commands::New(opts)) => {
            process_subcommand::new(&opts, ad)?;
            try_auto_sort(ad);
            alias_dirs::update_shmarks_file(shmarks_filepath.as_ref(), ad)?;
        }
        Some(Commands::Rm(opts)) => {
            process_subcommand::rm(&opts, ad)?;
            alias_dirs::update_shmarks_file(shmarks_filepath.as_ref(), ad)?;
        }
        Some(Commands::Ls(opts)) => process_subcommand::ls(&opts, ad),
        Some(Commands::Sort(opts)) => {
            process_subcommand::sort(&opts, ad);
            alias_dirs::update_shmarks_file(shmarks_filepath.as_ref(), ad)?;
        }
        _ => process_subcommand::none(&opts, ad)?,
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
