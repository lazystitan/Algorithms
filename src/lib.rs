pub struct Test {
    pub name: String,
}

pub mod for_test {
    pub fn zip_test() {
        let a = [1, 2, 3];
        let b = [4, 5, 6, 7];

        let c = a.iter().zip(b.iter());

        for (a, b) in c {
            println!("{}-{}", a, b);
        }
    }

    pub fn for_tag_test() {
        'outer: for i in 0..5 {
            for j in 0..25 {
                if j == 10 {
                    continue 'outer;
                }
                println!("i:{}, j:{}", i, j);
            }
        }
    }

    pub struct Line(pub i32);

    impl Iterator for Line {
        type Item = (i32);

        fn next(&mut self) -> Option<Self::Item> {
            if self.0 > 10 {
                None
            } else {
                self.0 += 1;
                Some(self.0)
            }
        }
    }
}

#[macro_use]
pub mod macro_test {

    macro_rules! several_times {
        ($($a : expr),+) => {
            {
                $(
                    println!("{}",$a);
                )+
            }
        };
    }

    #[cfg(test)]
    mod test {
//        use super::*;

        #[test]
        fn test() {
            several_times!(1,2,3);
            assert!(false);
        }
    }
}