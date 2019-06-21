mod boyer_moore;
mod kmp;
mod prime;
mod rabin_karp;
mod search;

pub use boyer_moore::Bm;
pub use kmp::Kmp;
pub use rabin_karp::RabinKarp;
pub use search::Search;
pub use prime::sieve::gen_prime;

fn search<T: Search>(string_match_algorithm: T, text: &String) -> usize {
    string_match_algorithm.search(text)
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{thread_rng, Rng};
    use std::io;

    #[test]
    fn main_test() {
        let pattern = "abc".to_string();
        let text = "aaabdec acbabc".to_string();
        let mut rng = thread_rng();
        let mut select: i32 = rng.gen();
        select = select % 4;
        let mut position;
        match select {
            1 => position = search(Kmp::new(pattern), &text),
            2 => position = search(Bm::new(pattern), &text),
            3 | _ => position = search(RabinKarp::new(pattern), &text),
        }

        assert_eq!(position, 11);
    }
}
