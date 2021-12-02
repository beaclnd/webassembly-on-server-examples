use std::error::Error;
use std::fs;
use wasmer::{imports, Instance, Module, Store};

fn main() -> Result<(), Box<dyn Error>> {
    let wasm_bytes = fs::read("./export.wasm")?; 
    let store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object)?;

    let add = instance.exports.get_native_function::<(i32, i32), i32>("add_2_numbers")?;
    let sum = add.call(12, 13)?;
    println!("The sum of the add function(from wasm module): {}", sum);

    let get_bool = instance.exports.get_native_function::<i32, i32>("get_bool")?;
    let res_bool = get_bool.call(true as i32)?;
    println!("The res of the get_bool function(from wasm module): {}", res_bool);

    Ok(())
}