(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func (param i32) (result i32)))
  (func $add_2_numbers (type 0) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.add)
  (func $get_reverse_bool (type 1) (param i32) (result i32)
    local.get 0
    i32.const 1
    i32.xor)
  (table (;0;) 1 1 funcref)
  (memory (;0;) 2)
  (global (;0;) (mut i32) (i32.const 66560))
  (export "memory" (memory 0))
  (export "add_2_numbers" (func $add_2_numbers))
  (export "get_reverse_bool" (func $get_reverse_bool)))
