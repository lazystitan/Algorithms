use crate::string_match::{prime, Search};

pub struct RabinKarp {
    pattern: String,
    pattern_hash: u64,
    pattern_len: usize,
    q: u64,
    r: u64,
    rm: u64,
}

impl RabinKarp {
    pub fn new(pattern: String) -> RabinKarp {
        let pattern_len = pattern.len();
        let r = 256;
        let max = 256u64.pow(pattern_len as u32);
        let q = long_random_prime(max);
        let pattern_hash = rk_hash(&pattern, pattern_len, q);
        let mut rm = 1;
        for i in 1..pattern_len {
            rm = (r * rm) % q;
        }

        RabinKarp {
            pattern,
            pattern_hash,
            pattern_len,
            q,
            r,
            rm,
        }
    }
}

impl Search for RabinKarp {
    fn search(&self, text: &String) -> usize {
        let text_as_bytes = text.as_bytes();
        let text_len = text.len();
        let mut text_hash = rk_hash(text, self.pattern_len, self.q);
        if self.pattern_hash == text_hash {
            return 0;
        }
        for i in self.pattern_len..text_len {
            text_hash = (text_hash + self.q
                - self.rm * text_as_bytes[i - self.pattern_len] as u64 % self.q)
                % self.q;
            text_hash = (text_hash * self.r + text_as_bytes[i] as u64) % self.q;
            if self.pattern_hash == text_hash {
                return i - self.pattern_len + 1;
            }
        }

        text_len
    }
}

fn rk_hash(pattern: &String, len: usize, q: u64) -> u64 {
    let pattern_as_bytes = pattern.as_bytes();
    let mut h = 0;
    for j in 0..len {
        h = (256 * h + pattern_as_bytes[j] as u64) % q;
    }
    h
}

fn long_random_prime(max: u64) -> u64 {
    prime::gen_prime_max(max)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_test() {
        let text = "acabc".to_string();
        let pattern = "abc".to_string();

        let bm = RabinKarp::new(pattern);
        let position = bm.search(&text);

        assert_eq!(position, 2);

        let pattern = "abcd".to_string();

        let bm = RabinKarp::new(pattern);
        let position = bm.search(&text);

        assert_eq!(position, text.len());

        let text = "where am i? what the fuck is this?".to_string();
        let pattern = "what".to_string();

        let bm = RabinKarp::new(pattern);
        let position = bm.search(&text);

        assert_eq!(position, 12);
    }
}
