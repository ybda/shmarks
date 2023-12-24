#![feature(absolute_path)]

mod app;
mod error;
mod normalize;
mod process_subcommand;
mod toml_parser;
mod util;
use crate::error::Result;
use crate::toml_parser::AliasesDirs;
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
    let shmarks_file_path = match env::var("SHMARKS_LIST_FILE") {
        Ok(value) => PathBuf::from(value),
        Err(_) => dirs::config_local_dir().ok_or_else(|| 
            Error::Msg(
                "Cannot resolve default config directory. Set 'SHMARKS_LIST_FILE' environment variable"
                .to_string()
            )
        )?,
    };

    if !shmarks_file_path.is_file() {
        return Err(Error::Msg(
            format!("'SHMARKS_LIST_FILE' is not a file. Provided path: {}", 
            shmarks_file_path.to_string_lossy())
        ));
    }

    let mut ads: AliasesDirs = {
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

        toml_parser::to_btreemap(&toml).map_err(|err| {
            format!(
                "Failed processing toml from '{}': {}",
                shmarks_file_path.to_str().unwrap(),
                err
            )
        })?
    };

    fn replace_contents_of_toml(ads: &AliasesDirs, shmarks_file_path: &PathBuf) -> Result<()> {
        let toml_new_string = toml::to_string_pretty(
            &toml_parser::to_toml(&ads)
        )?;
    
        util::replace_contents_of_file(&shmarks_file_path, &toml_new_string).map_err(|e| {
            format!("Failed replacing contents of '{}': {}", shmarks_file_path.to_string_lossy(), e)
        })?;
        
        Ok(())
    }

    let matches = app::matches();

    match matches.subcommand() {
        Some(("new", sub_m)) => {
            process_subcommand::new(&sub_m, &mut ads)?;
            replace_contents_of_toml(&ads, &shmarks_file_path)?;
        }
        Some(("rm", sub_m)) => {
            process_subcommand::rm(&sub_m, &mut ads)?;
            replace_contents_of_toml(&ads, &shmarks_file_path)?;
        }
        Some(("ls", sub_m)) => process_subcommand::ls(&sub_m, &mut ads),
        _ => process_subcommand::none(&matches, &ads, &shmarks_file_path)?,
    }

    Ok(())
}
