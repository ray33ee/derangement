# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### To Do
- Figure out any other functions the derangement may need
- Add any relevant traits
- Implement `ToString` and `Display` which converts the vector `_map` into cyclic form and into a string
  - Also implement `FromString`?
- Add function to convert a permutation (maybe find a crate with permutations) into a derangement
- Implement `serde` traits

### Unfinished Ideas
- Is there a better way to partition the shuffled set? 
 - Not sure if the current method has an even distribution (initial partition sizes may be larger than the latter ones. Though this may be offset by the fact that the list is shuffled initially)

## [0.1.0] - 2021-11-16
### Added
- Derangement class with get, underlying map and inverse functions
- Added tests