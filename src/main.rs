extern crate libc;

mod algorithms;
#[macro_use]
mod language_learn;
mod practice;

use std::rc::Rc;
use rand::{thread_rng, Rng};
use algorithms::sort::{SortTrait, MergeSort};
use core::borrow::Borrow;


#[derive(Debug, PartialOrd, PartialEq)]
struct Integer(i32);

fn main() {
    let mut vector = Vec::with_capacity(100);
    let mut rng = thread_rng();
    for _ in 0..100 {
        let num = rng.gen_range(0, 1000);
        vector.push(Rc::new(Integer(num)));
    }

    MergeSort::sort(&mut vector);
    assert!(MergeSort::is_sorted(&vector));

    let mut vector = Vec::new();
    for i in 0..3 {
        vector.push(Some(Integer(i)));
    }
    let i =  (&mut vector[1]).take().unwrap();
    println!("{:?}", i);
    println!("{:?}", vector);
//    let mut i2 = vector[1].unwrap();

}

