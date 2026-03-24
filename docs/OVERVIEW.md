# us-state-info-rs

`us-state-info-rs` is a Rust library providing a comprehensive enumeration of all U.S. state two-letter abbreviations. It offers convenient methods for converting these abbreviations to and from string representations, with optional Serde integration for robust serialization and deserialization.

## Architecture

The core of the library is the `StateAbbr` enum, which statically defines all 50 US state two-letter abbreviations as variants (e.g., `StateAbbr::AL`, `StateAbbr::TX`). It implements the `std::fmt::Display` trait, allowing `StateAbbr` variants to be easily formatted as their corresponding two-letter string representations.

The `std::str::FromStr` trait is also implemented, enabling robust parsing of string inputs (which are automatically converted to uppercase) into `StateAbbr` enum variants. This parsing logic handles invalid inputs by returning a `ParseStateAbbrError`.

For data serialization and deserialization, the library offers optional integration with the `serde` crate. When the `serde` feature is enabled, `StateAbbr` implements `serde::Serialize` and `serde::Deserialize`, allowing it to be seamlessly used within data structures that are serialized to or deserialized from various formats (e.g., JSON, YAML).

## Key Files

*   **`Cargo.toml`**: Defines the project metadata, including the package name (`us-state-info-rs`), version (`0.1.1`), author, description, and Apache-2.0 license. It specifies an optional `serde` dependency, allowing users to enable serialization/deserialization capabilities.
*   **`README.md`**: Provides a high-level overview of the library, including its purpose, build status and documentation badges, a quick usage example, and a cautionary note regarding its development status.
*   **`src/lib.rs`**: This is the main source file containing the `StateAbbr` enum definition. It includes the implementations for `std::fmt::Display` and `std::str::FromStr`, the custom `ParseStateAbbrError` type, and the conditional `serde::Serialize` and `serde::Deserialize` implementations. This file also hosts the library's unit tests.

## How to Run

To use `us-state-info-rs` in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
us-state-info-rs = "0.1" # Or the latest version
```

To enable Serde integration, specify the `serde` feature:

```toml
[dependencies]
us-state-info-rs = { version = "0.1", features = ["serde"] }
```

Example usage in your Rust code:

```rust
use us_state_info_rs::{StateAbbr, ParseStateAbbrError};
use std::str::FromStr; // Required for .parse()

fn main() -> Result<(), ParseStateAbbrError> {
    // Creating an enum variant
    let iowa = StateAbbr::IA;
    println!("Iowa abbreviation: {}", iowa); // Output: IA

    // Parsing from a string (case-insensitive)
    let colorado: StateAbbr = "CO".parse()?;
    println!("Colorado abbreviation: {}", colorado); // Output: CO

    let lowercase_new_york: StateAbbr = "ny".parse()?;
    println!("New York abbreviation: {}", lowercase_new_york); // Output: NY

    // Handling an invalid abbreviation
    let invalid_state = "XX".parse::<StateAbbr>();
    match invalid_state {
        Ok(_) => println!("This should not happen!"),
        Err(e) => println!("Error parsing state: {}", e), // Output: Error parsing state: invalid state abbreviation
    }

    // Using with Serde (if feature "serde" is enabled in Cargo.toml)
    #[cfg(feature = "serde")]
    {
        use serde::{Serialize, Deserialize};
        use serde_json;

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct MyData {
            home_state: StateAbbr,
        }

        let data = MyData { home_state: StateAbbr::WA };
        let json_string = serde_json::to_string(&data).unwrap();
        println!("Serialized data: {}", json_string); // Output: {"home_state":"WA"}

        let deserialized_data: MyData = serde_json::from_str(&json_string).unwrap();
        println!("Deserialized data: {:?}", deserialized_data); // Output: MyData { home_state: WA }
        assert_eq!(data, deserialized_data);
    }

    Ok(())
}
```

## How to Test

The library includes unit tests for its core functionality (Display, FromStr) and Serde integration. To run them, navigate to the project root and execute:

```bash
cargo test
```

To run tests specifically with the `serde` feature enabled (which includes the Serde-specific serialization/deserialization tests):

```bash
cargo test --features serde
```