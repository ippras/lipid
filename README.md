# Readme

* **u**: Unsaturated (any unsaturated: olefinic or acetylenic) {Triple:None}
* **o**: Olefinic (any olefinic: cis or trans) {Triple:Some(false);Parity:None}
* **c**: Cis {Triple:Some(false);Parity:Some(false)}
* **t**: Trans {Triple:Some(false);Parity:Some(true)}
* **a**: Acetylenic {Triple:Some(true)}

## Use

## Test

`cargo test --features=polars/timezones`