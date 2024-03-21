<!-- markdownlint-disable-file MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

Nothing yet

## [0.1.13] - 2024-03-21

### Added

- Add colors for default, current and removed directories in colored print with directories
- Flag to print marks with directories without color (`--plain`)
- README.md: `+nightly` in install command
- README.md: fzf jumper function

## [0.1.12] - 2024-03-17

### Added

- Batching for rm command (now you can pass multiple directories or aliases at once)

### Changed

- README.md
- Upgrade dependencies
- Change deprecated method IndexMap.remove to new IndexMap.shift_remove
- Apply clippy and formatting

## [0.1.11] - 2024-01-25

### Fixed

- Bottleneck in printing aliases with directories

## [0.1.10] - 2024-01-25

### Added

- rustfmt.toml

### Changed

- Upgrade dependencies
- README.md
- Code refactor 

### Fixed

- Error handling

## [0.1.9] - 2023-12-31

### Added

- Information in Cargo.toml.
- This file (CHANGELOG.md).
- Restriction of alias replacement by default (--force flag for new subcommand)

[0.1.13]: https://github.com/ybda/shmarks/commits/v0.1.13
[0.1.12]: https://github.com/ybda/shmarks/commits/v0.1.12
[0.1.11]: https://github.com/ybda/shmarks/commits/v0.1.11
[0.1.10]: https://github.com/ybda/shmarks/commits/v0.1.10
[0.1.9]: https://github.com/ybda/shmarks/commits/v0.1.9