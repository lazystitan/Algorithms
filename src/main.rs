mod algorithms;
#[macro_use]
mod language_learn;
mod practice;

use rand::{thread_rng, Rng};

struct ListNode {
    value : i32,
    next : Option<Box<ListNode>>
}

fn main() {
    let header =  ListNode {
        value : 1,
        next : Some(Box::new(ListNode {
            value : 2,
            next : Some(Box::new(ListNode {
                value : 3,
                next :None
            }))
        }))
    };

    let p = header.next.unwrap();

    println!("{}",p.value);
    println!("{}", header.value);
    println!("{}",p.value);
}

