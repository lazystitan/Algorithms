extern crate rand;
use rand::prelude::*;

mod kmp;
mod boyer_moore;
mod rabin_karp;
mod prime;

fn main() {
    let mut rng = thread_rng();
    let mut flag_low = false;
    let mut flag_high = false;
    for _ in 0..10000 {
        let x = rng.gen_range(0,10);
        if x == 0  {
            flag_low = true;
            break;
        } else if x == 10 {
            flag_high = true;
            break;
        }
    }

    if flag_low { println!("low"); }
    if flag_high { println!("high"); }
}
