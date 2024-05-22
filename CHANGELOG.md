# Change Log

Notable changes to the "Quality of Life" crate will be documented in this file.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.

## [0.1.10] - 2024-05-22

### Added

- RecurrentBTreeMap<T> BTreeMap that value Type is itself.

- RecurrentHashMap<T> HashMap that value Type is itself.

- PushOrInsert for BTreeMap

- unit tests for PushOrInsert for HashMap

- unit tests for AddOrInsert for HashMap

- unit tests for AddOrInsert for BTreeMap

### Source Reorganization

- moved `AddOrInsert` impls into ther own files

- moved `PushOrInsert` impls into ther own files

## [0.1.9] - 2024-04-05

### Added

- GetManyMut implementation for `Vec`s

### Source Reorganization

- Moved `BTreeMap` impl of `GetManyMut` into "get_many_mut\btreemap.rs"

- Renamed "unwrap_or_return_mod.rs" to "unwrap_or_return.rs"
