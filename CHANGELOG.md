# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1]
### Added
- Playground link in README

## [0.3.0] - 2024-12-10
### Changed
- All syntax is now Urdu inspired -> reference README for details

## [0.2.4] - 2024-11-23
### Fixed
- REPL mode didn't allow for cursor movement with arrow keys

### Added
- Coloring of print values (yellow for numbers and bool), (blue for functions and instances)

## [0.2.3] - 2024-11-21
### Added
- Logo to README

## [0.2.2] - 2024-11-20 
### Fixed
- File extension checker was checking incorrectly

## [0.2.1] - 2024-11-19
### Added
- README badges

## [0.2.0] - 2024-11-19
### Added
- Changelog file

### Changed
- Only `.qlm` files can be run
- CLI improvements with clap
- CLI can run raw strings of code
- README examples updated with new syntax
- README speed updated with test using build version of Qalam
- Syntax for variable declarations is now `shai` instead of `niyya` (e.g. `shai a = 1;`)
- Syntax for if statements is now `itha` instead of `shart` (e.g. `itha(condition) { // do something } illa { // something else }`)

### Fixed
- REPL mode remembers previous inputs by using shared interpreter

## [0.1.0] - 2024-08-21
### Added
- Initial release
