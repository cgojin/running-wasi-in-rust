use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

struct State {
    message: String,
    wasi: WasiCtx,
}

fn main() -> Result<()> {
    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |state: &mut State| {
        println!("Get WasiCtx, {}", state.message);
        &mut state.wasi
    })?;

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(
        &engine,
        State {
            message: String::from("all instances share this context and message"),
            wasi,
        },
    );

    // Load module
    let module = Module::from_file(&engine, "target/wasm32-wasi/debug/wasi.wasm")?;

    // Instantiate module
    linker.module(&mut store, "", &module)?;

    // Get the `default export`, it is `_start`, and run it
    linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ())?;

    /*
    // Get the `_start`, and run it
    linker
        .get(&mut store, "", "_start")
        .unwrap()
        .into_func()
        .unwrap()
        .typed::<(), ()>(&mut store)?
        .call(&mut store, ())?;

    // Get the `_start`, and run it
    let instance = linker.instantiate(&mut store, &module)?;
    instance
        .get_typed_func::<(), ()>(&mut store, "_start")?
        .call(&mut store, ())?;
    */

    Ok(())
}
