extern crate time;

use std::collections::VecDeque;

mod string_match;
mod math;
mod basic_type;
mod sort;

fn main() {
    for i in (1..10).rev() {
        println!("{}",i);
    }
}

trait Test {
    fn prt() {
        println!("ok");
    }

    fn test() {
        Self::prt();
    }
}


