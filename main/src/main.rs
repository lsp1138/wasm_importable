// main module that imports an "extern" library exported
// from the 'importable' library


extern "C" {
    fn add(x: i32, y: i32) -> i32;
}

fn main() {
    unsafe{
        println!("Calling add(1, 1) in imported module: {}", add(1,1) );
    }
}
