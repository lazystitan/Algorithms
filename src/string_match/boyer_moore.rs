pub struct Bm {
    pattern : String,
    right : Vec<i32>
}

impl Bm {
    pub fn new(pattern : String) -> Bm {
        let pattern_as_bytes = pattern.as_bytes();
        let len = 256;
        let mut right = vec![-1;len];
        for j in 0..pattern.len() {
            right[pattern_as_bytes[j] as usize] = j as i32;
        }

        eprintln!("{:?}",right);

        Bm {
            pattern,
            right
        }
    }

    pub fn search(&self, text : &String) -> usize {
        let text_as_bytes = text.as_bytes();
        let pattern_as_bytes = self.pattern.as_bytes();
        let n = text.len();
        let m = self.pattern.len();

        let mut skip=0;
        let mut i = 0;
        while i <= n-m {
            skip = 0;
            eprintln!("i:{}",i);
            for j in (0..m).rev() {
                eprintln!("j:{}",j);
                if pattern_as_bytes[j] != text_as_bytes[i+j] {
                    skip = j as i32 - self.right[text_as_bytes[i+j] as usize];
                    if skip < 1 { skip = 1; }
                    break;
                }
            }
            if skip == 0 { return i }
            i += skip as usize;
        }
        n
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_test() {
        let text = "acabc".to_string();
        let pattern = "abc".to_string();

        let bm = Bm::new(pattern);
        let position = bm.search(&text);

        assert_eq!(position, 2);

        let pattern = "abcd".to_string();

        let bm = Bm::new(pattern);
        let position = bm.search(&text);

        assert_eq!(position, text.len());
        let text = "where am i? what the fuck is this?".to_string();
        let pattern = "what".to_string();

        let bm = Bm::new(pattern);
        let position = bm.search(&text);

        assert_eq!(position, 12);

    }

}