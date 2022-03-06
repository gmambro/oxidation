# My personal list of useful Rust docs and crates.

## Examples
  * [Basic CLI with Tokio](cli_example)
  * [GitHub Actions CI Workflow](.github/workflows/ci-rust.yaml)

   ---

## Cli 

 * [indicatif](https://crates.io/crates/indicatif) - A progress bar and cli reporting library for Rust 
 * [structopt](https://crates.io/crates/structopt) — Parse command line argument by defining a struct. Based on [clap](https://crates.io/crates/clap) command line parser.
 * [termion](https://crates.io/crates/termion) - Termion is a pure Rust, bindless library for low-level handling, manipulating and reading information about terminals.

## Configuration

  * [dotenv](https://crates.io/crates/dotenv) - This library is meant to be used on development or testing environments in which setting environment variables is not practical. It loads environment variables from a .env file, if available, and mashes those with the actual environment variables provided by the operative system.


## Development tools
* [dtolnay/cargo-expand](https://github.com/dtolnay/cargo-expand) — Expand macros in your source code
* [clippy](https://crates.io/crates/clippy) — Rust lints
* [RLS (Rust Language Server)](https://github.com/rust-lang/rls) — A server that runs in the background, providing IDEs, editors, and other tools with information about Rust programs
* [rustfmt](https://github.com/rust-lang/rustfmt) — A Rust code formatter
* [rustup](https://github.com/rust-lang/rustup) — The Rust toolchain installer
* [tarpaulin](https://crates.io/crates/cargo-tarpaulin) — A code coverage tool designed for Rust. On MacOS run it with `docker run --security-opt seccomp=unconfined -v "${PWD}:/volume" xd009642/tarpaulin`
* [Tokio Console](https://github.com/tokio-rs/console) - A debugger for async rust! 


## Docs and articles

 * Getting started
   * [The Rust programming language](https://doc.rust-lang.org/book/) - The Book!
   * [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) - Learn Rust by solving exercises.
   * [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
 * Articles 
   * [24daysofrust](https://zsiciarz.github.io/24daysofrust/index.html) - The "24 days of Rust" article series. Old (2016) but still interesting.
 * Using Rust
   * [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
   * [Rust patterns](https://rust-unofficial.github.io/patterns/)

## Error handling
 * [miette](https://crates.io/crates/miette) A diagnostic library for Rust. It looks interesting especially for error handling in applications, and provide them with to get fancy report outputs. Think as an alternative to `anyhow`.
 * [snafu](https://crates.io/crate/snafu) - SNAFU is a library to easily assign underlying errors into domain-specific errors while adding context.
   * [Error handling guidelins](https://github.com/influxdata/influxdb_iox/blob/main/docs/style_guide.md#errors) from [InfluxDB IOx](https://github.com/influxdata/influxdb_iox)

## Logging

 * [tracing](https://crates.io/crates/tracing) A scoped, structured logging and diagnostics system.
   * [tracing-subscriber](https://crates.io/crates/tracing-subscriber)  Utilities for implementing and composing `tracing` subscribers. 

## RPC

 * [tonic](https://crates.io/crates/tonic) A native RustgRPC client & server implementation with async/await support. 
