# us-state-info-rs
United States Individual State Information
[![Docs]](https://docs.rs/us-state-info-rs/badge.svg)
[![Apache-2 licensed](https://img.shields.io/crates/l/us-state-info-rs.svg)](./LICENSE)
[![CI](https://github.com/calvinbrown085/us-state-info-rs/workflows/Rust/badge.svg)](https://github.com/calvinbrown085/us-state-info-rs/actions?query=workflow%3ARust)

A Rust implementation of the 2 letter state abbreviations for the USA.

## Example
```
use us_state_info_rs::StateAbbr;

let iowa_state_abbr = StateAbbr::IA;
format!("{}", iowa_state_abbr);
```


CAUTION: This application is still in _heavy_ development. Please use at your own risk.
