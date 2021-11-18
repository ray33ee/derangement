# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### To Do
- Add features for `To` and `From` for the `Permutation` struct
- Make `serde` optional

### Unfinished Ideas
- Is there a better way to partition the shuffled set? 
 - Not sure if the current method has an even distribution (initial partition sizes may be larger than the latter ones. Though this may be offset by the fact that the list is shuffled initially)

## [0.1.3] - 2021-11-18
### Added
- Published version to crates.io

## [0.1.2] - 2021-11-17
### Added
- `apply` method to apply a permutation from one slice into another
- `Display` trait which displays the permutation in cyclic form
- `Serde` support
- `TryFrom<&[usize]>` implemented with a special `Result` and our own `ErrorKind`
- Tests added for new features
- Improved docs quality

## [0.1.1] - 2021-11-17
### Added
- Extra fields into Cargo.toml for crates.io

## [0.1.0] - 2021-11-16
### Added
- Derangement class with get, underlying map and inverse functions
- Added tests