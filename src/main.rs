use rand::Rng;
use std::error::Error;
use wasmer::{imports, wat2wasm, Function, Instance, Module, NativeFunc, Store};

fn main() -> Result<(), Box<dyn Error>> {
    let wasm_bytes = wat2wasm(
        br#"
    (module
  (type $no_args_no_rets_t (func (param) (result)))
  (type $no_args_with_rets_t (func (param) (result i32)))
  (import "env" "say_hello" (func $say_hello (type $no_args_no_rets_t)))
  (import "env" "generate_sum_parameter" (func $gen_sum_param (type $no_args_with_rets_t)))
  (global $value2 (mut i32) (i32.const -1))
  (func $run (type $no_args_no_rets_t)
    (call $say_hello))
  (func $sum (param $value1 i32) (result i32) (local $v2 i32)
        (global.set $value2 (call $gen_sum_param))
        (local.set $v2 (global.get $value2)) 
        local.get $value1
        local.get $v2
        i32.add
    )
  (func $get_param2 (param) (result i32)
        global.get $value2
    )
  (export "get_param2" (func $get_param2))
  (export "sum" (func $sum))
  (export "run" (func $run)))"#,
    )?;

    // println!("wasm_bytes': {:?}", wasm_bytes);

    let store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;
    fn say_hello_world() {
        println!("Hello, world!");
    }
    // fn generate_sum_parameters() -> (i32, i32) {
    //     (rand::thread_rng().gen_range(1..101), rand::thread_rng().gen_range(1..101))
    // }
    fn generate_sum_parameter() -> i32 {
        rand::thread_rng().gen_range(1..101)
    }
    let import_object = imports! {
        "env" => {
            "say_hello" => Function::new_native(&store, say_hello_world),
            "generate_sum_parameter" => Function::new_native(&store, generate_sum_parameter),
        }
    };
    let instance = Instance::new(&module, &import_object)?;
    let run_func: NativeFunc<(), ()> = instance.exports.get_native_function("run")?;
    let sum_func = instance.exports.get_native_function::<i32, i32>("sum")?;
    let get_param2_from_global_func = instance.exports.get_native_function::<(), i32>("get_param2")?;

    run_func.call()?;
    let param1 = generate_sum_parameter();
    let sum_res = sum_func.call(param1)?;
    let param2 = get_param2_from_global_func.call()?;
    println!(
        "call function sum from wasm in the host: {} + {} = {}",
        param1,
        param2,
        sum_res,
    );

    Ok(())
}
