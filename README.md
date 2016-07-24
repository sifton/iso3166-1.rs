[travis-badge]: https://img.shields.io/travis/zeyla/iso3166-1.rs.svg?style=flat-square
[travis]: https://travis-ci.org/zeyla/iso3166-1.rs
[license-badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
[license]: https://opensource.org/licenses/ISC

[![travis-badge][]][travis] [![license-badge][]][license]

# iso3166-1.rs

Rust crate for ISO 3166-1 data.

[Documentation](http://docs.austinhellyer.me/iso3166_1)


### What is ISO 3166-1

> ISO 3166-1 is part of the ISO 3166 standard published by the International
> Organization for Standardization (ISO), and defines codes for the names of
> countries, dependent territories, and special areas of geographical interest.
>
> -- [Wikipedia](https://en.wikipedia.org/wiki/ISO_3166-1)


### Installation

Add the following dependency to your `Cargo.toml`:

```toml
iso3166_1 = "^1.0"
```

And include it in your project:

```rust
extern crate iso3166_1;
```

### Examples

Retrieve all country codes:

```rust
let countries = iso3166_1::all();
```


Retrieve a country code by its alpha2 code:

```rust
let country = iso3166_1::alpha2("AF").unwrap();
```


Retrieve a country code by its alpha3 code:

```rust
let country = iso3166_1::alpha3("ATA").unwrap();
```


Retrieve a country code by its name:

```rust
let country = iso3166_1::name("Angola").unwrap();
```


Retrieve a country code by its numeric number:

```rust
let country = iso3166_1::num("016").unwrap();
```


Retrieve country codes by a range of their numeric numbers:

```rust
// Getting all values between `100` and `300`:
iso3166_1::num_range(Some("100"), Some("300"));

// Getting all values from `400` and beyond:
iso3166_1::num_range(Some("400"), None);

// Getting all values up to `500`:
iso3166_1::num_range(None, Some("500"));

// Getting no values, if that's your thing:
iso3166_1::num_range(None, None);
```


### License

License info in [LICENSE.md]. Long story short, ISC.

[LICENSE.md]: https://github.com/zeyla/iso3166-1.rs/blob/master/LICENSE.md
