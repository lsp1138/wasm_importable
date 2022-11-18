//use importable::add;

#[link(name = "importable")]
extern "C" {
    fn add(x: i32, y: i32) -> i32;
}

fn main() {
    unsafe{
        println!("Hello, world! {}", add(1,1) );
    }
}
