<!-- markdownlint-disable-file MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Batching for rm command (now you can pass multiple directories or aliases at once)

### Changed

- README.md
- Upgrade dependencies
- Change deprecated method IndexMap.remove to new IndexMap.shift_remove

## [0.1.11] - 2023-01-25

### Fixed

- Bottleneck in printing aliases with directories

## [0.1.10] - 2023-01-25

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

[0.1.11]: https://github.com/ybda/shmarks/commits/v0.1.11
[0.1.10]: https://github.com/ybda/shmarks/commits/v0.1.10
[0.1.9]: https://github.com/ybda/shmarks/commits/v0.1.9