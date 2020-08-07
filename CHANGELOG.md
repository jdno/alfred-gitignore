# Changelog

The changelog documents all notable changes to this project. The format is based
on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) as defined
by [Cargo](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field)
and [Rust](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md).

<!-- next-header -->

## [Unreleased]

### Added

- Add template names as section headings to `.gitignore` file

## [2.0.1] - 2020-05-10

### Fixed

- Fix `Path to repository does not exist` error for fresh installations ([#4](https://github.com/jdno/alfred-gitignore/issues/4))

## [2.0.0] - 2020-04-26

### Changed

- Rewrite workflow in [Rust](https://rust-lang.org)

### Removed

- Remove dependency on `git` executable in path

## [1.1.0] - 2017-11-26

### Fixed

- Update dependency to fix bug in [macOS Sierra and later](https://github.com/deanishe/alfred-workflow/issues/111)

## [1.0.0] - 2015-11-25

### Added

- Release a workflow that can create `.gitignore` files from Alfred

<!-- next-url -->

[unreleased]: https://github.com/jdno/alfred-gitignore/compare/v2.0.1...HEAD
[2.0.1]: https://github.com/jdno/alfred-gitignore/compare/v2.0.0...v2.0.1
[2.0.0]: https://github.com/jdno/alfred-gitignore/compare/v1.1.0...v2.0.0
[1.1.0]: https://github.com/jdno/alfred-gitignore/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/jdno/alfred-gitignore/releases/tag/1.0.0
