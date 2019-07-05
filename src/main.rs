extern crate libc;

mod algorithms;
#[macro_use]
mod language_learn;
mod practice;



#[derive(Debug, PartialOrd, PartialEq)]
struct Integer(i32);

fn main() {
    let mut vector = vec![Integer(1),Integer(2),Integer(3)];
    vector = vector.into_iter().map(|x| Integer(x.0 * 2)).collect::<Vec<Integer>>();
    println!("{:?}", vector);
}

