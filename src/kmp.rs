
pub struct Kmp {
    pattern: String,
    dfa: Vec<Vec<usize>>
}

impl Kmp {
    pub fn new(pattern : String) -> Kmp{
        let pattern_as_bytes = pattern.as_bytes();
        let len = pattern.len();

        eprintln!("{}",len);

        let height = 256;
        let mut dfa:Vec<Vec<usize>> = vec![vec![0;len];height];

        eprintln!("{:?}",dfa[0]);

        let mut x = 0;
        let mut c= pattern_as_bytes[0] as usize;
        dfa[c][0] = 1;

        for j in 1..(pattern.len()-1) {
            c = pattern_as_bytes[j] as usize;
            for i in 0..256 {
                dfa[i][j] = dfa[i][x];
            }
            dfa[c][j] = j+1;
            x = dfa[c][x] as usize;
        }

        c = pattern_as_bytes[pattern.len()-1] as usize;
        dfa[c][pattern.len()-1] = pattern.len();
        Kmp {
            pattern,
            dfa
        }
    }

    pub fn show_dfa(&self) {
        for line in &self.dfa {
            println!("{:?}",line);
        }
    }

    pub fn search(&self, text : &String) -> usize
    {
        let mut j = 0;
        let mut position= 0;
        let text_as_bytes = text.as_bytes();

//        for line in &self.dfa {
//            eprintln!("{:?}",line);
//        }

        for i in 0..text.len() {
            eprintln!("{}",j);
            j = self.dfa[text_as_bytes[i] as usize][j] as usize;
            position = i;
            if j >= self.pattern.len() {
                break;
            }
        }
        if j == self.pattern.len() {
            position - self.pattern.len() + 1
        } else {
            text.len()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_test() {
//        let text = "acabc".to_string();
//        let pattern = "abc".to_string();
//
//        let kmp = Kmp::new(pattern);
//        let position = kmp.search(&text);
//
//        assert_eq!(position, 2);
//
//        let pattern = "abcd".to_string();
//
//        let kmp = Kmp::new(pattern);
//        let position = kmp.search(&text);
//
//        assert_eq!(position, text.len());

        let text = "where am i? what the fuck is this?".to_string();
        let pattern = "what".to_string();

        eprintln!("{}",pattern.len());

        let kmp = Kmp::new(pattern);
        kmp.show_dfa();
        let position = kmp.search(&text);

        assert_eq!(position, 12);

    }

}