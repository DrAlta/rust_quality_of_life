# Change Log

Notable changes to the "Quality of Life" crate will be documented in this file.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.
## [0.1.10] - 2024-07-19

### Added

- InnerIter adds a .inner_iter() method which will return a iter that if the Option is Some will iter over what you would iter over if you called `.unwrap().into_iter()` or nothing if the option is None

## [0.1.9] - 2024-04-05

### Added

- GetManyMut implementation for `Vec`s

### Source Reorganization

- Moved `BTreeMap` impl of `GetManyMut` into "get_many_mut\btreemap.rs"

- Renamed "unwrap_or_return_mod.rs" to "unwrap_or_return.rs"
