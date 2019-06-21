extern crate libc;

mod algorithms;
mod language_learn;
mod practice;

use cc_rs_libc_test::{ another_function, show_some_content };
use std::thread::sleep;
use std::time::Duration;
use std::thread;

fn call(input : i32) -> i32 {
    unsafe {
        another_function(input) as i32
    }
}

fn call_show() {
    unsafe {
        show_some_content();
    }
}

fn main() {
    thread::spawn(|| call_show());
    let input = 4;
    let output = call(input);
    println!("{} * 2 = {}", input, output);
    sleep(Duration::from_secs(2));
    println!("hello,long long long");
}
