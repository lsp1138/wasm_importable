## Create a WASM module that imports an exported function from another WASM module

Clone this repo and cd into `wasm_importable`

## Importable (library)

`cd importable`

build the "importable.wasm" file

`cargo build --target wasm32-unknown-unknown --release`

## Main

`cd ../main`

build the "main.wasm" file

`cargo build --target wasm32-unknown-unknown --release`

Check that the wasm files are built in /main.. and importable/target/wasm32-unknown-unknown/release/importable.wasm

Check the out put of the WASM.wat files (can you wasm-tools https://lib.rs/crates/wasm-tools and run `wasm-tools print module.wasm` ) contain the exported `add` and the imported `add` function.

Importable (.wat)

```
(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (func $add (;0;) (type 0) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.add
  )
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global (;1;) i32 i32.const 1048576)
  (global (;2;) i32 i32.const 1048576)
  (export "memory" (memory 0))
  (export "add" (func $add))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
)
```

Main (.wat) - just top of file

```
(module
  (type (;0;) (func))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (param i32) (result i32)))
  (type (;4;) (func (param i32 i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32) (result i32)))
  (type (;6;) (func (param i32 i32 i32)))
  (type (;7;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;8;) (func (param i32) (result i64)))
  (type (;9;) (func (param i32 i32 i32 i32)))
  (type (;10;) (func (result i32)))
  (type (;11;) (func (param i32 i32 i32 i32 i32)))
  (type (;12;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (type (;13;) (func (param i32 i32 i32 i32 i32 i32 i32)))
  (type (;14;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;15;) (func (param i64 i32 i32) (result i32)))
  (import "env" "add" (func $add (;0;) (type 4)))
  (func $_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h5a4d6040272ba004E.llvm.3651666977264313783 (;1;) (type 3) (param i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    ...
    ...
```


# Test in a http client

In a terminal

`cd /path/to/wasm_importable/`

and then run a test http server, f.ex:

`python3 -m http.server`

go to a browser and open `test_import_wasm_html` and check the console if the add function has worked







