extern crate iso3166_1;

fn main() {
    // Retrieve a `Vec` of `CountryCode`s whose numeric values are between `100`
    // and `300`.
    let countries1 = iso3166_1::num_range(Some("100"), Some("300"));

    // Retrieve a `Vec` of `CountryCode`s whose numeric values are greater than
    // or equal to `400`.
    let countries2 = iso3166_1::num_range(Some("400"), None);

    // Retrieve a `Vec` of `CountryCode`s whose numeric values are less than or
    // equal to `500`.
    let countries3 = iso3166_1::num_range(None, Some("500"));
    
    // Consequentially, passing None to both the `from` and `to` parameters will
    // give you no countries.
    let countries4 = iso3166_1::num_range(None, None);
    assert!(countries4.len() == 0); // true
}
