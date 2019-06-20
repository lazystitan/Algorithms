extern crate libc;

use algorithms::another_function;

fn call(input : i32) -> i32 {
    unsafe {
        another_function(input) as i32
    }
}

fn main() {
    let input = 4;
    let output = call(input);
    println!("{} * 2 = {}", input, output);
    println!("hello");
}
