use crate::kmp::Kmp;

mod kmp;
mod bm;

fn main() {
    for i in (0..4).map(|i| i * 2) {
        println!("{}",i);
    }
}
