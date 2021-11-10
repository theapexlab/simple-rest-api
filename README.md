# Simple REST API written in Rust

Simple Web service with two endpoints. The endpoints throw `BadRequest` error with a 20% probability on every request.

## Available endpoints

- [GET] `/second-power?base=value`

  The endpoint returns the second power of the base number.

  It returns the newly calculated number in string format or the `BadRequest` error.

- [GET] `/third-power?base=value`

  The endpoint returns the third power of the base number.

  It returns the newly calculated number in string format or the `BadRequest` error.

## Commands

Build: `cargo build`

Start: `cargo run`

Release build: `cargo build --release`

Release start: `cargo run --release`
