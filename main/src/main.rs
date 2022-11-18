// main module that imports an "extern" library exported
// from the 'importable' library

// the below will import the module as "importable" if uncommented, else it will belong
// to the 'env' module by default in the .wasm file
//#[link(wasm_import_module="importable")]
extern "C" {
    fn add(x: i32, y: i32) -> i32;
}

fn main() {
    unsafe{
        println!("Calling add(1, 1) in imported module: {}", add(1,1) );
    }
}
