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

# run linker example that linking two module
cargo run --example linker1
cargo run --example linker2
```

## Running wasi by wasmtime

```sh
# install wasmtime, first only
curl https://wasmtime.dev/install.sh -sSf | bash
# or install wasmtime by cargo
cargo install wasmtime-cli

# run wasi by wasmtime
wasmtime target/wasm32-wasi/debug/wasi.wasm

# run wasi by invoking `_start`
wasmtime --invoke _start target/wasm32-wasi/debug/wasi.wasm

# install wasm-tools, first only
cargo install wasm-tools

# show export methods
wasm-tools print target/wasm32-wasi/debug/wasi.wasm | grep "(export"
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (export "__main_void" (func $__main_void))
```

## Running wasi by cargo-wasi

```sh
# install cargo-wasi, first only
cargo install cargo-wasi

# run wasi by cargo-wasi
cargo wasi run -p wasi
```

## References

- [wasmtime](https://github.com/bytecodealliance/wasmtime)
