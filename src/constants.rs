pub const ENV_VAR_SHMARKS_LIST_PATH: &str = "SHMARKS_LIST_PATH";
pub const ENV_VAR_SHMARKS_AUTO_SORT: &str = "SHMARKS_AUTO_SORT";
pub const ENV_VAR_SHMARKS_DEFAULT_ALIAS: &str = "SHMARKS_DEFAULT_ALIAS";

pub const SHMARKS_DEFAULT_FILENAME: &str = "shmarks.toml";

pub fn ls_alias_style_normal() -> nu_ansi_term::Style {
    nu_ansi_term::Color::LightGreen.bold()
}
pub fn ls_alias_style_current() -> nu_ansi_term::Style {
    nu_ansi_term::Color::Blue.bold()
}
pub fn ls_alias_style_default() -> nu_ansi_term::Style {
    nu_ansi_term::Color::Cyan.bold()
}
pub fn ls_alias_style_removed() -> nu_ansi_term::Style {
    nu_ansi_term::Color::DarkGray.bold()
}

pub const LONG_LIST_PRINT_NUMBER_OF_SPACES: usize = 3;
