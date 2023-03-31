// A new instance is created for each call.

use wasmtime::*;
fn main() -> anyhow::Result<()> {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let mut linker = Linker::new(&engine);

    // Create a Command that attempts to count the number of times it is run, 
    // but is foiled by each call getting a new instance.
    let wat = r#"
    (module
        (global $counter (mut i32) (i32.const 0))
        (func (export "_start")
            (global.set $counter (i32.add (global.get $counter) (i32.const 1)))
        )
        (func (export "read_counter") (result i32)
            (global.get $counter)
        )
    )
    "#;
    let module = Module::new(&engine, wat)?;
    linker.module(&mut store, "commander", &module)?;

    let run = linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .clone();
    run.call(&mut store, ())?;
    run.call(&mut store, ())?;
    run.call(&mut store, ())?;

    let read_counter = linker
        .get(&mut store, "commander", "read_counter")
        .unwrap()
        .into_func()
        .unwrap()
        .typed::<(), i32>(&store)?;
    let count = read_counter.call(&mut store, ())?;
    println!("{}", count);
    assert_eq!(
        read_counter.call(&mut store, ())?, 0,
        "a Command should get a fresh instance on each invocation"
    );

    let wat = r#"
    (module
        (import "commander" "_start" (func $commander_start))
        (import "commander" "read_counter" (func $commander_read_counter (result i32)))
        (func (export "run") (result i32)
            call $commander_start
            call $commander_start
            call $commander_start
            call $commander_read_counter
        )
    )
    "#;
    let module = Module::new(&engine, wat)?;
    linker.module(&mut store, "", &module)?;
    let run = linker
        .get(&mut store, "", "run")
        .unwrap()
        .into_func()
        .unwrap()
        .typed::<(), i32>(&store)?;

    let count = run.call(&mut store, ())?;
    println!("{}", count);
    assert_eq!(
        count, 0,
        "a Command should get a fresh instance on each invocation"
    );

    Ok(())
}
