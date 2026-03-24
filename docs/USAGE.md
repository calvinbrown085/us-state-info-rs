# US-State-Info-RS Developer Usage Guide

`us-state-info-rs` is a Rust library providing an enumeration for the 2-letter abbreviations of all United States states. It offers convenient methods for displaying these abbreviations as strings and parsing them from case-insensitive string inputs, with optional Serde support for serialization and deserialization.

## Architecture

The library's core is the `StateAbbr` enum, which encapsulates each US state's two-letter abbreviation. This enum is designed for ease of use by implementing the `std::fmt::Display` trait for conversion to string representations and the `std::str::FromStr` trait for robust, case-insensitive parsing from string inputs. An associated `ParseStateAbbrError` type is provided for `FromStr` failures. Additionally, conditional compilation enables `serde::Serialize` and `serde::Deserialize` implementations for `StateAbbr` when the `serde` feature is activated, allowing for seamless integration with serialization frameworks like JSON.

## Key Files

*   **`Cargo.toml`**: Defines the project's metadata (name, version, authors, license), dependencies, and development dependencies. It declares `serde` as an optional dependency, enabling serialization/deserialization functionality via a feature flag.
*   **`src/lib.rs`**: Contains the entire public API of the `us-state-info-rs` crate. This includes the `StateAbbr` enum definition, its implementations for `Display` and `FromStr` (with `ParseStateAbbrError`), and the optional `Serialize`/`Deserialize` implementations for `StateAbbr` when the `serde` feature is enabled.

## How to Run

This is a Rust library meant to be included as a dependency in other Rust projects.

1.  **Add to your project's `Cargo.toml`:**
    ```toml
    [dependencies]
    us-state-info-rs = "0.1" # Use the latest published version
    ```
    To enable Serde support for serialization and deserialization:
    ```toml
    [dependencies]
    us-state-info-rs = { version = "0.1", features = ["serde"] }
    ```
2.  **Build your project:**
    ```bash
    cargo build
    ```

## Environment Variables

This library does not require any specific environment variables for its operation.

## How to Test

The project includes unit tests for its core functionality.

1.  **Run all tests:**
    ```bash
    cargo test
    ```
    This command will execute tests for the `Display` and `FromStr` implementations. If the `serde` feature is enabled in your `[dev-dependencies]` (as it is in the project's own `Cargo.toml`), it will also run tests for `Serialize` and `Deserialize` functionality.