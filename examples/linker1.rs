// Linking two module.

use wasmtime::*;
fn main() -> anyhow::Result<()> {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let mut linker = Linker::new(&engine);

    // This defines the `instance1::run` name for our next module to use.
    let wat = r#"
        (module
            (func (export "add") (param i32 i32) (result i32)
                local.get 0
                local.get 1
                i32.add
            )
        )
    "#;
    let module = Module::new(&engine, wat)?;
    linker.module(&mut store, "instance1", &module)?;

    let wat = r#"
        (module
            (import "instance1" "add" (func $add (param i32 i32) (result i32)) )
            (func (export "run") (result i32)
                i32.const 1
                i32.const 2
                call $add
            )
        )
    "#;
    let module = Module::new(&engine, wat)?;
    let instance = linker.instantiate(&mut store, &module)?;

    let run = instance.get_typed_func::<(), i32>(&mut store, "run")?;
    let result = run.call(&mut store, ())?;
    assert_eq!(result, 3);
    println!("{}", result);

    Ok(())
}
