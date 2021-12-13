use std::error::Error;
use std::fmt;
use std::fs;
use std::process::{self, Command};
use wasmer::{
    imports, Array, ChainableNamedResolver, Function, FunctionType, Instance, Memory, MemoryType,
    Module, Store, Type, WasmPtr, RuntimeError
};
use wasmer_wasi::WasiState;

#[derive(Debug, Clone, Copy)]
struct WrongParamFromWasm(i32);
impl fmt::Display for WrongParamFromWasm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for WrongParamFromWasm {}

fn main() -> Result<(), Box<dyn Error>> {
    // compile source code from import.c to import.wasm
    let build_output = Command::new("make").arg("import").output()?;
    if build_output.stderr.len() > 0 {
        println!("stderr: {}", String::from_utf8_lossy(&build_output.stderr));
        process::exit(1);
    }

    let wasm_bytes = fs::read("./import.wasm")?;
    let store = Store::default();
    let memory = Memory::new(&store, MemoryType::new(2, Some(7), false))?;
    let memory_clone1 = memory.clone();
    let memory_clone2 = memory.clone();
    let my_print_func_type = FunctionType::new(vec![Type::I32, Type::I32], vec![]);
    let my_print_func = Function::new(&store, &my_print_func_type, move |args| {
        let message_ptr = args[0].unwrap_i32();
        let message_length = args[1].unwrap_i32();
        let wasm_ptr = WasmPtr::<u8, Array>::new(message_ptr as u32);
        let message = wasm_ptr
            .get_utf8_string(&memory_clone1, message_length as u32)
            .unwrap();
        println!("{}", message);
        Ok(vec![])
    });
    let get_string_func_type =
        FunctionType::new(vec![Type::I32, Type::I32, Type::I32, Type::I32], vec![]);
    let get_string_func = Function::new(&store, &get_string_func_type, move |args| {
        let param_ptr = WasmPtr::<u8, Array>::new(args[0].unwrap_i32() as u32);
        let param_length = args[1].unwrap_i32() as u32;
        let param = param_ptr
            .get_utf8_string(&memory_clone2, param_length)
            .unwrap();
        let result_ptr = WasmPtr::<u8, Array>::new(args[2].unwrap_i32() as u32);
        let result_length_ptr = WasmPtr::<i32>::new(args[3].unwrap_i32() as u32);
        let set_result = |info: &[u8]| {
            let cell = result_length_ptr.deref(&memory_clone2).unwrap();
            cell.set(info.len() as i32);
            let cells = result_ptr
                .deref(&memory_clone2, 0, info.len() as u32)
                .unwrap();
            for i in 0..info.len() {
                cells[i].set(info[i]);
            }
        };
        match param.as_str() {
            "string1" => {
                let info = b"123456";
                set_result(info);
            }
            "string2" => {
                let info = b"987654321";
                set_result(info);
            }
            _ => {
                RuntimeError::raise(Box::new(WrongParamFromWasm(-1)));
            }
        }
        Ok(vec![])
    });
    fn get_pi() -> f64 {
        3.1415926
    }
    fn get_r() -> f64 {
        2.0
    }
    let get_pi_func = Function::new_native(&store, get_pi);
    let get_r_func = Function::new_native(&store, get_r);
    let module = Module::new(&store, wasm_bytes)?;
    let import_object = imports! {
        "env" => {
            "printThis" => my_print_func,
            "getString" => get_string_func,
            "getPi" => get_pi_func,
            "getR" => get_r_func,
            "memory" => memory,
        }
    };
    let mut wasi_env = WasiState::new("import_example").finalize()?;
    let wasi_import_object = wasi_env.import_object(&module)?;
    let instance = Instance::new(&module, &import_object.chain_front(&wasi_import_object))?;

    let do_somethings = instance
        .exports
        .get_native_function::<(i32, i32), ()>("doSomethings")?;
    let malloc_in_module = instance
        .exports
        .get_native_function::<i32, i32>("mallocInModule")?;
    let free_in_module = instance
        .exports
        .get_native_function::<i32, ()>("freeInModule")?;
    let param2_val = b"I am param2";
    let param2 = malloc_in_module.call(param2_val.len() as i32)?;
    let param2_wasm_ptr = WasmPtr::<u8, Array>::new(param2 as u32);
    // Use exported memory because the memory variable has been moved above
    let memory_exported = instance.exports.get_memory("memory")?;
    let cells = param2_wasm_ptr
        .deref(&memory_exported, 0, param2_val.len() as u32)
        .unwrap();
    for i in 0..param2_val.len() {
        cells[i].set(param2_val[i]);
    }
    let param1 = 42;
    do_somethings.call(param1, param2)?;
    free_in_module.call(param2)?;
    Ok(())
}
