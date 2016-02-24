extern crate iso3166_1;

fn main() {
    // Retrieve a `Vec` of all `CountryCode`s.
    let countries = iso3166_1::all();
}
