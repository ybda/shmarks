shmarks
------

# Usage
```
$ shmarks --help
Directory bookmarks for the shell

Usage: shmarks [OPTIONS]
       shmarks <COMMAND>

Commands:
  new   Create new mark [aliases: n]
  rm    Remove mark. Removes mark of current dir if no options provided [aliases: r]
  ls    List all marks [aliases: l]
  help  Print this message or the help of the given subcommand(s)

Options:
  -a, --alias <alias>  Alias of the directory to jump into
  -e, --edit           Edit marks in '$EDITOR'
  -h, --help           Print help
  -V, --version        Print version
```

## Code to add in your .zshrc (should work with little changes in other shells as well)
```
alias s='shmarks'
alias p='s l -d'  # Aesthetic print
export SHMARKS_LIST_FILE="$HOME/.config/mylinuxcfg/shmarks.toml"
f() {
    if [[ $# -eq 0 ]]; then
        cd "$(shmarks)"
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

## How to use

Jump by alias

```bash
> f myalias
```

Edit marks file in preferred $EDITOR

```bash
> shmarks -e 
```

Save current dir (pwd) to shmarks

```bash
> shmarks new myalias
```

Save specified dir to shmarks 

```bash
> shmarks new myalias -d /my/dir
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

List all saved marks like "/bin/ls -l" in column with dirs showed

```bash
> shmarks ls -d
```

## Note
- By default shmarks.toml located in your_config_dir/shmarks.toml. You could override it with $SHMARKS_LIST_FILE
- It requires nightly Rust only because of the "std::path::absolute" 

## Inspired by
- [huyng/bashmarks](https://github.com/huyng/bashmarks)