mod algorithms;
#[macro_use]
mod language_learn;
mod practice;

use rand::{thread_rng, Rng};


#[derive(Debug, PartialOrd, PartialEq)]
struct Integer(i32);

fn main() {
    let mut rng = thread_rng();
    let mut v = Vec::new();

    for _ in 0..10 {
        v .push(rng.gen_range(0, 100));
    }

    println!("{:?}",v);
}

