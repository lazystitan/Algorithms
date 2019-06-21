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