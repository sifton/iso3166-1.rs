[travis-badge]: https://img.shields.io/travis/taiyaeix/iso3166-1.rs.svg
[travis]: https://travis-ci.org/taiyaeix/iso3166-1.rs
[license-badge]: https://img.shields.io/badge/license-ISC-blue.svg
[license]: https://opensource.org/licenses/ISC

[![travis-badge][]][travis] [![license-badge][]][license]

# iso3166-1.rs

Rust crate for ISO 3166-1 data.


### What is ISO 3166-1

> ISO 3166-1 is part of the ISO 3166 standard published by the International
> Organization for Standardization (ISO), and defines codes for the names of
> countries, dependent territories, and special areas of geographical interest.
>
> -- [Wikipedia](https://en.wikipedia.org/wiki/ISO_3166-1)


### Installation

Add the following dependency to your Cargo.toml:

```toml
iso3166_1 = "*"
```

And include it in your project:

```rust
extern crate iso3166_1;
```

### Examples

Retrieve all country codes:

```rust
extern crate iso3166_1;

fn main() {
    let countries = iso3166_1::all().unwrap();
}
```


Retrieve a country code by its alpha2 code:

```rust
extern crate iso3166_1;

fn main() {
    let country = iso3166_1::alpha2("AF").unwrap();
}
```


Retrieve a country code by its alpha3 code:

```rust
extern crate iso3166_1;

fn main() {
    let country = iso3166_1::alpha3("ATA").unwrap();
}
```


Retrieve a country code by its name:

```rust
extern crate iso3166_1;

fn main() {
    let country = iso3166_1::name("Angola").unwrap();
}
```


Retrieve a country code by its numeric number:

```rust
extern crate iso3166_1;

fn main() {
    let country = iso3166_1::num("016").unwrap();
}
```


Retrieve country codes by a range of their numeric numbers:

```rust
extern crate iso3166_1;

fn main() {
    // Getting all values between 100 and 300:
    iso3166_1::num_range(Some("100"), Some("300")).unwrap();

    // Getting all values from 400 and beyond:
    iso3166_1::num_range(Some("400"), None).unwrap();

    // Getting all values up to 500:
    iso3166_1::num_range(None, Some("500")).unwrap();

    // Getting no values, if that's your thing:
    iso3166_1::num_range(None, None).is_none();
}
```


### License

License info in `LICENSE.md`. Long story short, ISC.
