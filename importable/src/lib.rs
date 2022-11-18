// export this function as external with no_mangle as "C"
// style function addressing

#[no_mangle]
pub extern fn add(x: i32, y: i32) -> i32 {
    x + y
}
