use clap::Command;
use clap::{command, Arg, ArgMatches};
use std::env;
use std::path::PathBuf;

pub fn matches() -> ArgMatches {
    command!()
        .about("Directory bookmarks for the shell")
        .arg(
            Arg::new("alias")
                .short('a')
                .long("alias")
                .exclusive(true)
                .help("Alias of the directory to jump into"),
        )
        .arg(
            Arg::new("edit")
                .short('e')
                .long("edit")
                .help("Edit marks in $EDITOR")
                .exclusive(true)
                .num_args(0),
        )
        .subcommand(
            Command::new("new")
                .visible_alias("n")
                .about("Create new mark")
                .arg(
                    Arg::new("alias")
                        .required(true)
                        .help("Alias of the directory to create"),
                )
                .arg(
                    Arg::new("directory")
                        .short('d')
                        .long("directory")
                        .value_parser(clap::value_parser!(PathBuf))
                        .help("Directory to mark")
                        .default_value("."),
                ),
        )
        .subcommand(
            Command::new("rm")
                .visible_alias("r")
                .alias("remove")
                .about("Remove mark. Removes mark of current dir if no options provided")
                .arg(
                    Arg::new("alias")
                        .short('a')
                        .long("alias")
                        .help("Alias of the directory to remove")
                        .conflicts_with("directory"),
                )
                .arg(
                    Arg::new("directory")
                        .short('d')
                        .long("directory")
                        .value_parser(clap::value_parser!(PathBuf))
                        .help("Directory to remove")
                        .conflicts_with("alias"),
                ),
        )
        .subcommand(
            Command::new("ls")
                .visible_alias("l")
                .alias("list")
                .about("List all marks")
                .arg(
                    Arg::new("directory")
                        .short('d')
                        .long("directory")
                        .help("Print directories as well")
                        .num_args(0),
                ),
        )
        .get_matches()
}
