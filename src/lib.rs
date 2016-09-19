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
// Originally by zeyla on GitHub.

mod codes;

pub use codes::all;

use std::num::ParseIntError;

/// Container for the data of each Country Code defined by ISO 3166-1,
#[derive(Clone, Debug)]
pub struct CountryCode<'a> {
    /// Two-character Alpha2 code
    pub alpha2: &'a str,
    /// Three-character Alpha3 code
    pub alpha3: &'a str,
    /// English short name of the country
    pub name: &'a str,
    /// NUmeric code of the country
    pub num: &'a str,
}

/// Returns the CountryCode with the given Alpha2 code if one exists.
pub fn alpha2<'a>(alpha2: &str) -> Option<CountryCode<'a>> {
    all().into_iter().find(|c| c.alpha2 == alpha2)
}

/// Returns the CountryCode with the given Alpha3 code if one exists.
pub fn alpha3<'a>(alpha3: &str) -> Option<CountryCode<'a>> {
    all().into_iter().find(|c| c.alpha3 == alpha3)
}

/// Returns the CountryCode with the given name if one exists.
pub fn name<'a>(name: &str) -> Option<CountryCode<'a>> {
    all().into_iter().find(|c| c.name == name)
}

/// Returns the CountryCode with the given number of one exists.
pub fn num<'a>(num: &str) -> Option<CountryCode<'a>> {
    all().into_iter().find(|c| c.num == num)
}

/// Returns a `Vec` of `CountryCode`s that have a numeric value within the range
/// of the `from` and `to` given. The from and to are optional, and can either
/// be `None` or `Some(&str)` for variations of the range wanted.
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
                     to: Option<&str>)
                     -> Result<Vec<CountryCode<'a>>, ParseIntError> {
    let from_do = from.is_some();
    let to_do = to.is_some();
    let from_val = try!(from.unwrap_or("0").parse::<i16>());
    let to_val = try!(to.unwrap_or("0").parse::<i16>());

    Ok(all().into_iter().filter(|code| {
        let num_as_int = code.num.parse::<i16>().unwrap();
        let gte = num_as_int >= from_val;
        let lte = num_as_int <= to_val;

        {
            if from_do && to_do {
                gte && lte
            } else if from_do {
                gte
            } else if to_do {
                lte
            } else {
                false
            }
        }}).collect())
}
