#![feature(absolute_path)]

use std::fs::File;
use clap::Parser;
mod aliases_dirs;
mod cli;
mod constants;
mod error;
mod normalize;
mod process_subcommand;
mod shmarks_file;
mod util;
use crate::aliases_dirs::AliasesDirs;
use crate::cli::{Cli, Commands};
use crate::error::Result;
use std::path::Path;
use std::process;

fn main() {
    run().unwrap_or_else(|e| {
        error::default_error_handler(&e, &mut std::io::stderr().lock());
        process::exit(1);
    })
}

fn run() -> Result<()> {
    let shmarks_filepath = shmarks_file::retrieve_filepath()?;

    if !shmarks_filepath.exists() {
        File::create(&shmarks_filepath)
            .map_err(|err| format!("Failed creating '{}': {}", &shmarks_filepath.to_string_lossy(), err))?;
    }

    let mut ad: AliasesDirs = shmarks_file::parse(&shmarks_filepath)?;

    process_args(&mut ad, &shmarks_filepath)?;

    Ok(())
}

fn process_args<P: AsRef<Path>>(ad: &mut AliasesDirs, shmarks_filepath: P) -> Result<()> {
    let opts = Cli::parse();

    match opts.command {
        Some(Commands::New(opts)) => {
            process_subcommand::new(&opts, ad)?;
            shmarks_file::update(shmarks_filepath.as_ref(), ad)?;
        }
        Some(Commands::Rm(opts)) => {
            process_subcommand::rm(&opts, ad)?;
            shmarks_file::update(shmarks_filepath.as_ref(), ad)?;
        }
        Some(Commands::Ls(opts)) => process_subcommand::ls(&opts, ad),
        _ => process_subcommand::none(&opts, &ad)?,
    }

    Ok(())
}
