# Corrosion

Experimental frontend framework using the Rust language.

## Local Dev

### Pre-requisites

- Cargo
- wasm-pack - https://rustwasm.github.io/wasm-pack/installer/

### Building

- Build wasm bundle `wasm-pack build --target web`
- Build static files `cargo run`

### Enabling Hot Reload

- Install [cargo-watch](https://lib.rs/crates/cargo-watch) | `cargo install cargo-watch`
- `cargo watch -x run`
