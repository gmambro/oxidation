My personal list of useful Rust docs and crates.

## Cli 

 * [structopt](https://crates.io/crates/structopt) — Parse command line argument by defining a struct

## Configuration

  * [dotenv](https://crates.io/crates/dotenv) - This library is meant to be used on development or testing environments in which setting environment variables is not practical. It loads environment variables from a .env file, if available, and mashes those with the actual environment variables provided by the operative system.

## Error handling

 * [snafu](https://crates.io/crate/snafu) - SNAFU is a library to easily assign underlying errors into domain-specific errors while adding context.
   * [Error handling guidelins](https://github.com/influxdata/influxdb_iox/blob/main/docs/style_guide.md#errors) from [InfluxDB IOx](https://github.com/influxdata/influxdb_iox)

## Logging

 * [tracing](https://crates.io/crates/tracing) A scoped, structured logging and diagnostics system.
   * [tracing-subscriber](https://crates.io/crates/tracing-subscriber)  Utilities for implementing and composing `tracing` subscribers. 
