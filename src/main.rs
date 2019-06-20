extern crate libc;

extern {
    fn another_function(input: libc::c_int) -> libc::c_int;
}

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
