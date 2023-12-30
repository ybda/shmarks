use clap::Command;
use clap::{command, Arg, ArgMatches};
use std::env;
use std::path::PathBuf;

pub const SUBCOMMAND_NEW: &str = "new";
pub const SUBCOMMAND_RM: &str = "rm";
pub const SUBCOMMAND_LS: &str = "ls";

pub const ARG_DIRECTORY: &str = "directory";
pub const ARG_EDIT: &str = "edit";
pub const ARG_ALIAS: &str = "alias";

const ARG_DIRECTORY_SHORT: char = 'd';
const ARG_EDIT_SHORT: char = 'e';
const ARG_ALIAS_SHORT: char = 'a';

pub fn matches() -> ArgMatches {
    command!()
        .args_conflicts_with_subcommands(true)
        .arg_required_else_help(true)
        .max_term_width(100)
        .arg(
            Arg::new(ARG_ALIAS)
                .long(ARG_ALIAS)
                .short(ARG_ALIAS_SHORT)
                .exclusive(true)
                .help("Alias of the directory to jump into"),
        )
        .arg(
            Arg::new(ARG_EDIT)
                .long(ARG_EDIT)
                .short(ARG_EDIT_SHORT)
                .help("Edit marks in '$EDITOR'")
                .exclusive(true)
                .num_args(0),
        )
        .subcommand(
            Command::new(SUBCOMMAND_NEW)
                .visible_alias("n")
                .about("Create new mark")
                .arg(
                    Arg::new(ARG_ALIAS)
                        .required(true)
                        .help("Alias of the directory to create"),
                )
                .arg(
                    Arg::new(ARG_DIRECTORY)
                        .long(ARG_DIRECTORY)
                        .short(ARG_DIRECTORY_SHORT)
                        .value_parser(clap::value_parser!(PathBuf))
                        .help("Directory to mark")
                        .default_value("."),
                ),
        )
        .subcommand(
            Command::new(SUBCOMMAND_RM)
                .visible_alias("r")
                .about("Remove mark. Removes mark of current dir if no options provided")
                .alias("remove")
                .arg(
                    Arg::new(ARG_ALIAS)
                        .long(ARG_ALIAS)
                        .short(ARG_ALIAS_SHORT)
                        .help("Alias of the directory to remove")
                        .conflicts_with("directory"),
                )
                .arg(
                    Arg::new(ARG_DIRECTORY)
                        .long(ARG_DIRECTORY)
                        .short(ARG_DIRECTORY_SHORT)
                        .value_parser(clap::value_parser!(PathBuf))
                        .help("Directory to remove")
                        .conflicts_with("alias"),
                ),
        )
        .subcommand(
            Command::new(SUBCOMMAND_LS)
                .visible_alias("l")
                .about("List all marks")
                .alias("list")
                .arg(
                    Arg::new(ARG_DIRECTORY)
                        .long(ARG_DIRECTORY)
                        .short(ARG_DIRECTORY_SHORT)
                        .help("Print directories as well")
                        .num_args(0),
                ),
        )
        .get_matches()
}
