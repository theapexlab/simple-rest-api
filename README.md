# Simple REST API written in Rust

The endpoints throw `BadRequest` error with a 20% probability on every request.

## Available endpoints

- [GET] `/second-power?<base>`</br>

  The endpoint returns the second power of the base number

- [GET] `/third-power?<base>`

  The endpoint returns the third power of the base number

## Commands

Build: `cargo build`

Start: `cargo run`

Release build: `cargo build --release`

Release start: `cargo run --release`
