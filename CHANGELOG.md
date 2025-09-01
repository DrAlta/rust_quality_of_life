# Change Log

Notable changes to the "Quality of Life" crate will be documented in this file.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.
## [0.1.20] - 2025-08-31

### Added

- assert_specimen!() like assert_eq!() but whoes messages makes it clear that you area testing te first argument againt the second arugment that is what it is supost to be equal to.

### Tod0

- make logy!() a proc macro, code is writen needs to be tested and does the stuff for rust ot allow it asa proc macro

## [0.1.19] - 2025-03-08

### Changed

- fixed opening and closing tags when Displaying Vecna

### Added

- unit test fot Display of Vecna

## [0.1.18] - 2025-03-08

### Added

- `impl<'a, T: std::fmt::Display>  From<&'a Vec<T>> for Vecna<'a, T>`

## [0.1.17] - 2025-02-01

### Added

- InsertOrInsert and impe for `BTreeMap<K, BTreeSet<V>>` and `HashMap<K, BTreeSet<V>>`

## [0.1.16] - 2024-09-24

### Added

- pout-debug features that enables a version of pout that prints the file and line number like logy

## [0.1.15] - 2024-09-23

-fixed a bug in BiHashMapIter where it would get stuck in a infinate loop.

## [0.1.14] - 2024-09-20

### Added

- implemented FromIterator<((O, I), V)> for BiHashMap<O, I, V>

## [0.1.13] - 2024-09-19

### Added

- added BiHashMapIter

- implemented IntoIterator for &'a BiHashMap<O, I, V>

- implemented FromIterator<((&'a O,&'b I), V)> for BiHashMap<O, I, V>

## [0.1.12] - 2024-09-19

### Added

- added BiHashMap

## [0.1.11] - 2024-08-05

### Added

- InnerIter trait add `.inner_iter()` method to Options of types that impl IntoIter, returns a iter that with iter over ther inner value if it is `Some()` otherwise it ithers over nothings.

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
