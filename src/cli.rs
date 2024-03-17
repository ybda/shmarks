use std::path::PathBuf;

use clap::{Args, Parser, ValueHint};

#[derive(Parser)]
#[command(version, about, long_about = None, args_conflicts_with_subcommands = true, arg_required_else_help = true)]
pub struct Cli {
    /// Alias of the directory to jump into.
    #[arg(short, long, exclusive = true)]
    pub alias: Option<String>,

    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(clap::Subcommand)]
pub enum Subcommand {
    New(NewOpts),
    Rm(RmOpts),
    Ls(LsOpts),
    Sort(SortOpts),
}

/// Create new mark. Creates mark for current directory by default.
#[derive(Args, Debug)]
#[command(visible_alias = "n")]
pub struct NewOpts {
    /// Alias of the directory to create.
    #[clap(required = true, value_hint = ValueHint::Other)]
    pub alias: String,

    /// Directory to mark [default: current directory]
    #[clap(value_hint = ValueHint::DirPath)]
    pub directory: Option<PathBuf>,

    /// Rewrite alias if exists.
    #[clap(short, long)]
    pub force: bool,
}

/// Remove mark(s). Removes mark of current dir if no args provided.
#[derive(Args, Debug)]
#[command(visible_alias = "r")]
pub struct RmOpts {
    /// Alias(es) of the directory to remove.
    #[clap(short, long, value_hint = ValueHint::Other, conflicts_with="directory",  num_args = 1.., value_delimiter = ' ')]
    pub aliases: Option<Vec<String>>,

    /// Directory(ies) to remove.
    #[clap(short, long, value_hint = ValueHint::DirPath, conflicts_with="alias", num_args = 1.., value_delimiter = ' ')]
    pub directories: Option<Vec<PathBuf>>,
}

/// List all marks.
#[derive(Args, Debug)]
#[command(visible_alias = "l", alias = "list")]
pub struct LsOpts {
    /// Print directories as well.
    #[clap(short, long)]
    pub directory: bool,
}

/// Sort shmarks file.
#[derive(Args, Debug)]
#[command(visible_alias = "s")]
pub struct SortOpts {
    /// Sort by directories.
    #[clap(short, long)]
    pub directory: bool,
}
