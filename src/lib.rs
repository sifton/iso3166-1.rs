// ISC License (ISC)
//
// Copyright (c) 2016, Austin Hellyer <hello@austinhellyer.me>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER
// RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF
// CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// What is ISO 3166-1?
//
// | ISO 3166-1 is part of the ISO 3166 standard published by the International
// | Organization for Standardization (ISO), and defines codes for the names of
// | countries, dependent territories, and special areas of geographical
// | interest.
// |
// | - [Wikipedia](http://en.wikipedia.org/wiki/ISO_3166-1)
//
// Originally by taiyaeix on GitHub.

mod codes;

use codes::country_codes;

/// Struct that contains the data for each Country Code defined by ISO 3166-1,
/// including the following pieces of information:
///
/// - `alpha2` - Two-character Alpha-2 code.
/// - `alpha3` - Three-character Alpha-3 code.
/// - `name` - English short name of country.
/// - `num` - Numeric code of country.
///
/// Derives from Clone and Debug.
#[derive(Clone, Debug)]
pub struct CountryCode<'a> {
    pub alpha2: &'a str,
    pub alpha3: &'a str,
    pub name: &'a str,
    pub num: &'a str,
}

/// Returns an `Option` of a `Vec` of all `CountryCode`s defined by ISO 3166-1.
///
/// # Examples
///
/// ```rust
/// let countries = iso3166_1::all();
/// ```
pub fn all<'a>() -> Vec<CountryCode<'a>> {
    country_codes()
}

/// Returns an `Option` of a `CountryCode` with the given alpha2 code.
///
/// # Examples
///
/// ```rust
/// let country = iso3166_1::alpha2("AF").unwrap();
/// ```
pub fn alpha2<'a>(alpha2: &str) -> Option<CountryCode<'a>> {
    let mut code_ret: Option<CountryCode> = None;

    for code in country_codes() {
        if code.alpha2 == alpha2 {
            code_ret = Some(code.clone());

            break;
        }
    }

    code_ret
}

/// Returns an `Option` of a `CountryCode` with the given alpha3 code.
///
/// # Examples
///
/// ```rust
/// let country = iso3166_1::alpha3("ATA").unwrap();
/// ```
pub fn alpha3<'a>(alpha3: &str) -> Option<CountryCode<'a>> {
    let mut code_ret: Option<CountryCode> = None;

    for code in country_codes() {
        if code.alpha3 == alpha3 {
            code_ret = Some(code.clone());

            break;
        }
    }

    code_ret
}

/// Returns an `Option` of a `CountryCode` with the given name.
///
/// # Examples
///
/// ```rust
/// let country = iso3166_1::name("Angola").unwrap();
/// ```
pub fn name<'a>(name: &str) -> Option<CountryCode<'a>> {
    let mut code_ret: Option<CountryCode> = None;

    for code in country_codes() {
        if code.name == name {
            code_ret = Some(code.clone());

            break;
        }
    }

    code_ret
}

/// Returns an `Option` of a `CountryCode` with the given numeric value.
///
/// # Examples
///
/// ```rust
/// let country = iso3166_1::num("016").unwrap();
/// ```
pub fn num<'a>(num: &str) -> Option<CountryCode<'a>> {
    let mut code_ret: Option<CountryCode> = None;

    for code in country_codes() {
        if code.num == num {
            code_ret = Some(code.clone());

            break;
        }
    }

    code_ret
}

/// Returns an `Option` of a `Vec` of `CountryCode`s that have a numeric value
/// within the range of the `from` and `to` given. The from and to are optional,
/// and can either be `None` or `Some(&str)` for variations of the range wanted.
///
/// # Examples
///
/// Getting all values between `100` and `300`:
///
/// ```rust
/// let countries = iso3166_1::num_range(Some("100"), Some("300"));
/// ```
///
/// Getting all values from `400` and beyond:
///
/// ```rust
/// let countries = iso3166_1::num_range(Some("400"), None);
/// ```
///
/// Getting all values up to `500`:
///
/// ```rust
/// let countries = iso3166_1::num_range(None, Some("500"));
/// ```
///
/// Getting no values, if that's your thing:
///
/// ```
/// let countries = iso3166_1::num_range(None, None);
/// ```
pub fn num_range<'a>(from: Option<&str>,
                     to: Option<&str>) -> Vec<CountryCode<'a>> {
    let mut codes: Vec<CountryCode> = vec![];

    let from_do: bool = from.is_some();
    let to_do: bool = to.is_some();

    let from_val: i16 = if from_do {
        from.unwrap().parse::<i16>().unwrap()
    } else {
        0
    };
    let to_val: i16 = if to_do {
        to.unwrap().parse::<i16>().unwrap()
    } else {
        0
    };

    for code in country_codes() {
        let num_as_int: i16 = code.num.parse::<i16>().unwrap();

        let gte: bool = num_as_int >= from_val;
        let lte: bool = num_as_int <= to_val;

        let matches: bool = if from_do && to_do {
            gte && lte
        } else if from_do {
            gte
        } else if to_do {
            lte
        } else {
            false
        };

        if matches {
            codes.push(code.clone());
        }
    }

    codes
}
