use std::process;
use crate::kmp::Kmp;

mod matrix;
mod kmp;

fn main() {
    let kmp = Kmp::new("ababac".to_string());
    kmp.show_dfa();
}
