use crate::error::{Error, Result};
use crate::normalize;
use crate::toml_parser::AliasesDirs;
use crate::util;
use clap::ArgMatches;
use std::env;
use std::path::PathBuf;
use std::process;

pub fn ls(m: &ArgMatches, toml_map: &mut AliasesDirs) {
    let alias_color = nu_ansi_term::Color::LightCyan;

    if m.get_flag("directory") {
        let color_len = alias_color.paint(".").to_string().len() - 1;
        let max_length = toml_map.keys().map(|s| s.len()).max().unwrap_or(0);

        for key in toml_map.keys() {
            let padding = max_length - key.len() + color_len;
            let formatted_string = format!(
                "{:<width$}   {}",
                alias_color.paint(key).to_string(),
                toml_map[key].to_string_lossy(),
                width = key.len() + padding
            );
            println!("{}", formatted_string);
        }
    } else {
        for (index, element) in toml_map.keys().enumerate() {
            if index < toml_map.keys().len() - 1 {
                print!("{} ", element);
            } else {
                print!("{}", element);
            }
        }
        println!();
    }
}

pub fn rm(m: &ArgMatches, toml_map: &mut AliasesDirs) -> Result<()> {
    if let Some(alias) = m.get_one::<String>("alias") {
        toml_map.remove(alias);
        return Ok(());
    }

    if let Some(directory) = m.get_one::<PathBuf>("directory") {
        let absolute_directory = normalize::abs_normalize_path(&directory)?;
        util::remove_first_value_from_map(toml_map, &absolute_directory);
        return Ok(());
    }

    // Handle default case when there's no args
    let pwd = normalize::abs_normalize_path(&PathBuf::from("."))?;
    util::remove_first_value_from_map(toml_map, &pwd);

    Ok(())
}

pub fn new(m: &ArgMatches, toml_map: &mut AliasesDirs) -> Result<()> {
    let alias_arg = m.get_one::<String>("alias").unwrap();
    let path_arg = m.get_one::<PathBuf>("directory").unwrap();
    let absolute_path_arg = normalize::abs_normalize_path(path_arg)?;

    toml_map.insert(alias_arg.to_string(), absolute_path_arg);
    Ok(())
}

pub fn none(m: &ArgMatches, toml_map: &AliasesDirs, shmarks_file_path: &PathBuf) -> Result<()> {
    if let Some(alias) = m.get_one::<String>("alias") {
        let dir_to_set = &toml_map.get(alias);
        if let Some(dir) = dir_to_set {
            println!("{}", dir.to_string_lossy());
        } else {
            return Err(Error::Msg(format!("Alias '{}' not found", alias)));
        }
    } else if m.get_flag("edit") {
        let editor = env::var("EDITOR").unwrap_or_else(|_| String::from("vi"));
        process::Command::new(&editor)
            .arg(&shmarks_file_path)
            .status()
            .expect("Failed to open the file in the editor");
    } else {
        let default_alias_name = "_default";
        if let Some(dir) = &toml_map.get(default_alias_name) {
            println!("{}", dir.to_string_lossy());
        } else {
            return Err(Error::Msg(format!(
                "No default directory was set, add alias '{}' to cd into default directory",
                default_alias_name
            )));
        }
    }
    Ok(())
}
