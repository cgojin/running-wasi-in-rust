# running-wasi-in-rust

Running wasi in Rust.

## Running wasi in Rust

```sh
# add wasm32-wasi target, first only
rustup target add wasm32-wasi

# build wasi
cargo build -p wasi --target wasm32-wasi

# run wasi in Rust
cargo run
```

## References

- [wasmtime](https://github.com/bytecodealliance/wasmtime)
