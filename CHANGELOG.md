# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Changed

### Added


## [1.0.0] - 2016-02-24

### Changed

- `all()` and `num_range()` now return a `Vec` of `CountryCode`s, and not an
  `Option` of a `Vec`. This was worthless because instead of a `Some` if there
  was at least one entry or a `None` if there were none, a `len()` check on the
  `Vec` can just be performed.

### Added


## [0.1.0] - 2016-01-13

Initial commit.


[Unreleased]: https://github.com/taiyaeix/iso3166-1.rs/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/taiyaeix/iso3166-1.rs/compare/v0.1.0...v1.0.0
[0.1.0]: https://github.com/taiyaeix/iso3166-1.rs/compare/b44021c...v0.1.0
