extern crate rand;

mod kmp;
mod boyer_moore;
mod rabin_karp;
mod prime;
mod search;
mod string_match;

use rand::prelude::*;
//use rabin_karp::RabinKarp;
//use search::Search;
use string_match::{RabinKarp, Search};

fn main() {
    let text = "where am i? what the fuck is this?".to_string();
    let pattern = "what".to_string();

    let bm = RabinKarp::new(pattern);
    let position = bm.search(&text);
}
