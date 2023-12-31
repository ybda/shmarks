shmarks
------

[![Crates.io](https://img.shields.io/crates/v/shmarks.svg)](https://crates.io/crates/shmarks)

## Getting started
![Example](https://i.imgur.com/lcbWcXr.png)

## Installation

1. **Install binary**

    ```bash
    > cargo install shmarks --locked
    ```

2. **Add code in your .zshrc** (should work with little changes in other shells as well)

    ```bash
    export SHMARKS_LIST_PATH="$HOME/.local/share/shmarks.toml"
    export SHMARKS_AUTO_SORT="d" # sort on adding new alias: a = by aliases, d = by directories, otherwise no sorting
    export SHMARKS_DEFAULT_ALIAS="default" # default alias to jump into if no alias name was provided
    alias s='shmarks'
    alias p='shmarks ls -d' # aesthetic print
    alias se="$EDITOR $SHMARKS_LIST_PATH" # edit shmarks
    # Jump by alias
    f() {
        if [[ $# -eq 0 ]]; then
            cd "$(shmarks -a "$SHMARKS_DEFAULT_ALIAS")"
        else
            cd "$(shmarks -a "$@")"
        fi
    }
    # Autocompletion stuff
    _shmarks_compzsh() {
        reply=($(shmarks ls))
    }
    compctl -K _shmarks_compzsh f
    ```

## Usage
```bash
> shmarks
Directory bookmarks for the shell.

Usage: shmarks [OPTIONS]
       shmarks <COMMAND>

Commands:
  new   Create new mark. Creates mark for current directory by default [aliases: n]
  rm    Remove mark. Removes mark of current dir if no args provided [aliases: r]
  ls    List all marks [aliases: l]
  sort  Sort shmarks file [aliases: s]
  help  Print this message or the help of the given subcommand(s)

Options:
  -a, --alias <ALIAS>  Alias of the directory to jump into
  -h, --help           Print help
  -V, --version        Print version
```

Jump by alias 'default' into default dir

```bash
> f
```

Jump by alias

```bash
> f myalias
```

Edit marks file in $EDITOR

```bash
> se
```

Save current dir (pwd) to shmarks and sort shmarks file if $SHMARKS_AUTO_SORT was set

```bash
> shmarks new myalias
```

```bash
> s n myalias
```

Save specified dir to shmarks 

```bash
> shmarks new myalias /my/dir
```

Remove current dir from shmarks

```bash
> shmarks rm
```

Remove alias specified by its directory path

```bash
> shmarks rm -d /my/dir
```

Remove alias specified by its name

```bash
> shmarks rm -a myalias
```

List all saved marks like plain GNU "ls" utility

```bash
> shmarks ls 
```

List all saved marks like "/bin/ls -l" in columns with dirs showed, colored 

```bash
> shmarks ls -d
```

```bash
> p
```

Sort shmarks by directories (alphabetical order)

```bash
> shmarks sort -d
```

### Note
- By default, shmarks.toml located in $XDG_DATA_HOME or $HOME/.local/share. You could override it with $SHMARKS_LIST_PATH
- It requires nightly Rust only because of the "std::path::absolute" to resolve relative paths that might not exist

### Inspired by
- [huyng/bashmarks](https://github.com/huyng/bashmarks)