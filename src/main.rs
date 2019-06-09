use std::{process, io};
use crate::kmp::Kmp;

mod kmp;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.pop();
    let number:usize = buffer.parse().unwrap();
    let mut array = vec![vec![0;3];3];
    println!("len:{}",number);
    array[1][2] = 2;
    println!("array:{:?}",array);
}
