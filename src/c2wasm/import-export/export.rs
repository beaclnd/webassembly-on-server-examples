use std::error::Error;
use std::fs;
use std::process::{self, Command};
use wasmer::{imports, Array, Instance, Module, Store, ValueType, WasmPtr};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct Simple_Str {
    offset: i32,
    length: i32,
}
unsafe impl ValueType for Simple_Str {}

fn main() -> Result<(), Box<dyn Error>> {
    // compile source code from export.c to export.wasm
    let build_output = Command::new("make").arg("export").output()?;
    if build_output.stderr.len() > 0 {
        println!("stderr: {}", String::from_utf8_lossy(&build_output.stderr));
        process::exit(1);
    }

    let wasm_bytes = fs::read("./export.wasm")?;
    let store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object)?;

    let add = instance
        .exports
        .get_native_function::<(i32, i32), i32>("add2Numbers")?;
    let sum = add.call(12, 13)?;
    println!("The sum of the add function(from wasm module): {}", sum);

    let get_reverse_bool = instance
        .exports
        .get_native_function::<i32, i32>("getReverseBool")?;
    let get_reverse_bool_param = true as i32;
    let reverse_bool = get_reverse_bool.call(get_reverse_bool_param)?;
    println!(
        "The res of the get_reverse_bool({}) function(from wasm module): {}",
        get_reverse_bool_param, reverse_bool
    );

    // read data from wasm module
    let memory = instance.exports.get_memory("memory")?;
    println!(
        "Memory in total: {} pages {} bytes",
        memory.size().0,
        memory.data_size()
    );
    let malloc_in_module = instance
        .exports
        .get_native_function::<i32, i32>("mallocInModule")?;
    let free_in_module = instance
        .exports
        .get_native_function::<i32, ()>("freeInModule")?;
    let get_simple_str_size = instance
        .exports
        .get_native_function::<(), i32>("getSimpleStrSize")?;
    let get_string = instance
        .exports
        .get_native_function::<i32, ()>("getStringWithPointerParam")?;
    let simple_str_size = get_simple_str_size.call()?;
    let offset = malloc_in_module.call(simple_str_size)?;
    get_string.call(offset)?;
    let simple_str_ptr = WasmPtr::<Simple_Str>::new(offset as u32);
    let derefed_ptr = simple_str_ptr
        .deref(&memory)
        .expect(&format!("bad pointer in {} for struct Simple_Str", offset));
    let simple_str: Simple_Str = derefed_ptr.get();
    println!("Got simple_str from wasm module {:?}", simple_str);
    let string_val_ptr = WasmPtr::<u8, Array>::new(simple_str.offset as u32);
    let string_val = string_val_ptr
        .get_utf8_string(&memory, simple_str.length as u32)
        .unwrap();
    println!("Got string from wasm module: {}", string_val);
    free_in_module.call(simple_str_ptr.offset() as i32)?;

    // write data to wasm module
    let string_to_write = b"Hello, I am from rust";
    let offset = malloc_in_module.call(string_to_write.len() as i32)?;
    let string_val_ptr = WasmPtr::<u8, Array>::new(offset as u32);
    let cells = string_val_ptr
        .deref(&memory, 0, string_to_write.len() as u32)
        .unwrap();
    for i in 0..string_to_write.len() {
        cells[i].set(string_to_write[i]);
    }
    let string_val = string_val_ptr
        .get_utf8_string(&memory, string_to_write.len() as u32)
        .unwrap();
    println!("Got string rust written into wasm moudle: {}", string_val);

    println!(
        "Memory in total: {} pages {} bytes",
        memory.size().0,
        memory.data_size()
    );

    // Clang doesn't support wasm globals from C/C++
    // we can only get the globals through the exported function instead of the exported global variables
    let get_global_number = instance
        .exports
        .get_native_function::<(), i32>("getGlobalNumber")?;
    println!("Got the global number: {}", get_global_number.call()?);

    Ok(())
}
