### Dependencies 
- Rust: setup Rust with rustup
- clang/llvm: v13.0.0+
- make
- wabt: https://github.com/WebAssembly/wabt 
- wasi-sdk: https://github.com/WebAssembly/wasi-sdk.git　(change --sysroot value in the Makefiles when using a different target localtion for wasi-sdk)
- libclang_rt.builtins-wasm32.a: put the file libclang_rt.builtins-wasm32.a(which can be found  in wasi-sdk) into clang's lib directory e.g. /usr/lib/clang/13.0.0/lib/wasi 

### Run Examples
Run JIT hello-world:
```bash
cargo run --example c2wasm-hello
```

Run JIT export:
```bash
cargo run --example c2wasm-export
```

Run JIT import:
```bash
cargo run --example c2wasm-import
```