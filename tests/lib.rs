// ISC License (ISC)
//
// Copyright (c) 2016, Zeyla Hellyer <zeylahellyer@gmail.com>
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
//
extern crate iso3166_1;

use iso3166_1::{all, alpha2, alpha3, name, num, num_range};

#[test]
fn get_all() {
    assert!(all().len() > 0);
}

// Ensure alpha2, alpha3, and num values are 2, 3, and 3 characters long,
// respectively.
#[test]
fn value_lengths() {
    let all = all();

    for val in all {
        assert!(val.alpha2.len() == 2);
        assert!(val.alpha3.len() == 3);
        assert!(val.num.len() == 3);
    }
}

#[test]
fn get_alpha2() {
    // Test an lpha2 that exists.
    assert!(alpha2("AF").is_some());

    // Test an alpha2 that does not exist.
    assert!(alpha2("AA").is_none());
}

#[test]
fn get_alpha3() {
    // Test an alpha3 that exists.
    assert!(alpha3("AFG").is_some());

    // Test an alpha3 that does not exist.
    assert!(alpha3("AAA").is_none());
}

#[test]
fn get_name() {
    // Test a name that exists.
    assert!(name("Afghanistan").is_some());

    // Test a name that does not exist.
    assert!(name("ABCDEF").is_none());
}

#[test]
fn get_num() {
    // Test a num that exists.
    assert!(num("004").is_some());

    // Test a num that does not exist.
    assert!(num("000").is_none());
}

#[test]
fn get_num_range() {
    // Test a Some-Some range with countries with nums within it.
    assert!(num_range(Some("001"), Some("005")).unwrap().len() > 0);
    // And with no countries within it.
    assert!(num_range(Some("001"), Some("003")).unwrap().len() == 0);

    // Test a None-Some range with countries with nums within it.
    assert!(num_range(None, Some("004")).unwrap().len() > 0);
    // And with no countries within it.
    assert!(num_range(None, Some("003")).unwrap().len() == 0);

    // Test a Some-None range with countries with nums within it.
    assert!(num_range(Some("001"), None).unwrap().len() > 0);
    // And with no countries within it.
    assert!(num_range(Some("1000"), None).unwrap().len() == 0);

    // Test a None-None range, which is always None.
    assert!(num_range(None, None).unwrap().len() == 0);
}

// Backwards compatibility tests.
#[test]
fn backwards_compat() {
    assert!(all().len() != 0);
}
