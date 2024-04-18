#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("rust_cpp_cxx/include/math_utils.h");
        fn hello();
        fn add(num_1: i32, num_2: i32) -> i32;
        fn multiply(num_1: i32, num_2: i32) -> i32;
        fn subtract(num_1: i32, num_2: i32) -> i32;
    }
}
fn main() {
    ffi::hello();
    let add: i32 = ffi::add(10, 5);
    let multiply = ffi::multiply(10, 5);
    let subtract = ffi::subtract(10, 5);

    println!("Add two: {}", add);
    println!("Multiply two: {}", multiply);
    println!("Subtract two: {}", subtract);
}
