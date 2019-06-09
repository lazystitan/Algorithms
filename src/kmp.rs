use std::io;
use std::io::Read;
use crate::matrix::Matrix;

pub fn main() -> Result<(),&'static str>
{
    let mut buffer = String::new();
    if let  Err(err) =  io::stdin().read_line(&mut buffer) {
        return Err("stdin error");
    }

    let a = buffer.split_whitespace().collect::<Vec<&str>>();

    for i in a {
        println!("{}",i)
    }

    Ok(())

}

pub struct Kmp {
    pattern: String,
    dfa: Matrix
}

impl Kmp {
    pub fn new(pattern : String) -> Kmp{
        let len = pattern.len();
        let height = 256;
        let mut dfa = Matrix::new(len,height);

        let mut x = 0;
        let mut c= pattern.as_bytes().get(0).unwrap();

        dfa.set(*c as usize, 0, 1);

        for j in 1..(pattern.len()-1) {
            c = pattern.as_bytes().get(j).unwrap();
            for i in 0..256 {
                println!("{}-{}-{}",i,j,x);
                dfa.set(i, j, dfa.get(i, x).unwrap());
            }
            dfa.set(*c as usize, j, j as i32 + 1);
            x = dfa.get(*c as usize, x).unwrap() as usize;
        }

        dfa.set(*pattern.as_bytes().get(pattern.len()-1).unwrap() as usize,pattern.len()-1 as usize, pattern.len() as i32);

        Kmp {
            pattern,
            dfa
        }
    }

    pub fn show_dfa(&self) {
        println!("{}",self.dfa);
    }

    pub fn search(&self, text : String) -> usize
    {
        let mut j = 0;
        let mut position= 0;
        for i in 0..text.len() {
            j = self.dfa.get(*text.as_bytes().get(i).unwrap() as usize, j).unwrap() as usize;
            position = i;
        }
        if j == self.pattern.len() {
            position - self.pattern.len()
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
        let text = "acabc".to_string();
        let pattern = "abc".to_string();

        let kmp = Kmp::new(pattern);
        let position = kmp.search(text);

        assert_eq!(position, 1);

    }

}