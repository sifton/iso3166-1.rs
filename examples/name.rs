#![allow(unused_variables)]

extern crate iso3166_1;

fn main() {
    // Retrieve a `CountryCode` given a name. This is an `Option`, so should be
    // handled appropriately.
    let country = iso3166_1::name("Angola");
}
