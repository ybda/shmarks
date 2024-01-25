pub const ENV_VAR_SHMARKS_LIST_PATH: &str = "SHMARKS_LIST_PATH";
pub const ENV_VAR_SHMARKS_AUTO_SORT: &str = "SHMARKS_AUTO_SORT";

pub const SHMARKS_DEFAULT_FILENAME: &str = "shmarks.toml";

pub fn ls_alias_style() -> nu_ansi_term::Style {
    nu_ansi_term::Color::LightGreen.bold()
}
pub const LS_ALIAS_STYLE_NUMBER_OF_SPACES: usize = 3;
